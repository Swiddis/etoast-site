---
layout: base.njk
title: "Performance Case Study: Cloc vs. Tokei"
author_date: February 13, 2025
---

# {{ title }}

{{ author_date }}

*This is a lightly edited repost of a [comment I wrote on an issue in the Tokei repository](https://github.com/XAMPPRocky/tokei/issues/917#issuecomment-2654971356).*
*Tokei and Cloc are both tools that count lines of code in a directory, filtering out comments and blank lines.*
*The original question was:*

> Hi. I think it would be pretty cool if you add to the readme something
> explaining how `tokei` is so blazing fast.
> 
> I did a quick comparison with `cloc` and `tokei` was 20 times faster. I know
> `cloc` was made in perl, but the language itself is probably not the only
> reason for that.
> 
> Anyway, I just think people would like a high-level explanation of that.

---

I was also curious (particularly after migrating from Cloc), so I did some
testing and investigation with a specific 2.4 million-line codebase I work with.

First thing's first: Cloc isn't parallel by default, you need to pass
`--processes=N` for parallelism. For fairness I ran Cloc with `--processes=4`
and Tokei with `RAYON_NUM_THREADS=4`. It turns out this doesn't help Cloc much
if at all (output from Fish's `time`):

```
$ time RAYON_NUM_THREADS=4 tokei ~/code/OpenSearch/
# ...
________________________________________________________
Executed in    1.53 secs    fish           external
   usr time    0.68 secs    0.25 millis    0.68 secs
   sys time    1.20 secs    2.50 millis    1.20 secs

$ time cloc --processes=4 ~/code/OpenSearch/
# ...
________________________________________________________
Executed in   64.04 secs    fish           external
   usr time   35.44 secs    0.21 millis   35.44 secs
   sys time   20.17 secs    1.53 millis   20.16 secs
```

Something that stood out to me even from here was that Cloc spent a lot more
time waiting on the system, which means it's not just the language itself, but
something about its file access pattern is dramatically less efficient. (Of
course, user time is also significantly slower).

Following my heart, I generated flamegraphs for Tokei:

![Flamegraph of a Tokei execution. The majority of the time is spent in many open-then-parse steps.](/static/img/perf-cloc-vs-tokei/flamegraph_tokei.svg)

And Cloc:

![Flame graph of a Cloc execution. Each individual step is visibile, the majority is file IO and `rm_comments`](/static/img/perf-cloc-vs-tokei/flamegraph_cloc.svg)

As a sanity test, these graphs roughly line up with the user/sys timing above.

In Cloc, we can see each individual step it's running laid out individually
(list all the files, then dedupe, then count everything). In `call_counter`,
it's clear that it's doing multiple passes over each file to count different
types of lines, with `rm_comments` in particular taking a while. We also can see
that it's spending the majority of its IO time on `is_binary`, which tests if a
file is a binary file or not (presumably to filter them out),
[some details on this algorithm on Bendersky's site](https://eli.thegreenplace.net/2011/10/19/perls-guess-if-file-is-text-or-binary-implemented-in-python).
It also is relying solely on Regex matching for all this parsing, which is
[famously slow in Perl](https://swtch.com/~rsc/regexp/regexp1.html)[^1]. Since
each step is fully independent, that means IO gets duplicated for each run.

In contrast, the Tokei flamegraph is dominated by Rayon's `execute`, where
individual `execute` calls are split roughly 65/35 into IO and parsing
respectively. Rayon is doing a lot of heavy lifting to keep threads
saturated[^2]. A lot of filtering happens in the directory walking (which is
also parallel and channel-driven in `src::utils::fs`), and the filtering is just
based on the file metadata, so there's no `is_binary` check at all. Each parse
call is done in one pass and relies on much more small-scale Regex. The regex
matching itself also will be [significantly faster than Perl's](https://burntsushi.net/regex-internals/).

Comparing them[^3]:
- Tokei is architected to only read each file once, and aggressively filters
  files before reading them if possible.
- Tokei has a more efficient parsing architecture than Cloc, doing less passes
  and more quickly.
- Rayon and `crossbeam_channel` do a lot of heavy lifting for efficient parallel
  execution, Cloc's ad-hoc parallel implementation simply doesn't compete (if
  it's enabled at all).

**tl;dr:** sane libraries and iterator-driven architecture.

[^1]: Cloc spends about 8x the entire run-time of Tokei just in Perl's built-in
    regex matching.
[^2]: I can't actually find any clear information on if Rayon's work stealing
    will handle blocking file syscalls? It doesn't look like the current
    implementation does async file IO, so it's possible we're wasting time
    waiting. Would it be any faster to do file IO async?
[^3]: This answer isn't necessarily complete because there's several other tools
    faster than Cloc as well. Comparing with them may be more interesting. I'm
    pleasantly surprised by how apparent the architectural tradeoffs are here,
    though.
