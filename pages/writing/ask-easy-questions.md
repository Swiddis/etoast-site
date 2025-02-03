---
layout: base.njk
title: Ask Easy Questions
author_date: February 2, 2025
tags:
  - blog
---

# {{ title }}

{{ author_date }}

<div align="right">
    <i>"One should never try to prove anything that is not almost obvious."</i>
    <br>
    <i>― Alexander Grothendieck</i>
</div>

For the longest time after entering the workforce, I struggled with asking questions. See, I grew up in a developer culture where [How To Ask Questions The Smart Way](https://www.catb.org/esr/faqs/smart-questions.html) was part of the local literary canon, so it usually happened that when someone explained something complicated and asked, “Any questions?” I would answer no, on the presumption I could research it and later ask a better question. This ended up causing a compounding cycle of not having enough time to catch up to what I needed to know and, in turn, not asking questions as I felt I was behind where I needed to be to ask them.

I’ve worked over time to undo that habit; however, in the long journey to reach that point, I found myself regularly asking, “What should I even ask?” The title spoils the answer: you should ask the easiest, most obvious questions you have. For the rest of this post, I’m going to view that same conclusion from three different vantage points.

### 1. The Most Obvious Response

As a pair of hobbies, I do [improv](https://en.wikipedia.org/wiki/Improvisational_theatre) and play [tabletop RPGs](https://en.wikipedia.org/wiki/Tabletop_role-playing_game). The premise is that a group of us gets on a stage or sits around a room or table and makes things up without any prior rehearsal. Maybe there’s a prompt or an established setting, but for the most part, you don’t have much planning available. One thing that I wanted to learn was how people who are good at these skills are able to, seemingly magically, know exactly what to say. Fairly often it was witty or emotional or otherwise left an impact. I’ve had a few different mentors for these skills, and I distinctly remember one time I asked one of them just that: “How do you know what to say?”

They answered, “Just say the most obvious response.”

So I started doing that. It took some getting used to, but I decided that for any given scene I would just enter with an empty head and let my instincts handle the rest. Weirdly enough, it worked: some of the best scenes I remember playing have come as the result of simply responding to unexpected events with the very first thing that came to my mind. After some more time, I figured out the magic trick: what’s obvious to you is, shockingly often, not obvious to others. As a whole, people have such diverse ideas, experiences, wit, and [recent encounters](https://en.wikipedia.org/wiki/Recency_bias) that whatever you think of when presented with a new idea is virtually guaranteed to be slightly different from what anyone else thinks in the same position. Those differences are where creativity comes from.

### 2. The Backyard Bird Chronicles

I want to share an excerpt from Amy Tan’s _The Backyard Bird Chronicles,_ where she recounts one of her experiences as part of a nature journaling club.

> On the second field trip, I met a teenage girl, who had recently turned thirteen. She was accompanied by her mother. We were standing next to a wide body of water in the Consumnes River Preserve twenty miles south of Sacramento. Before us were waterfowl and wading birds. Sandhill cranes flew overhead, three thousand of them on their way to a nearby marsh field, outside of the town of Grove. Her journal pages were dense with fast watercolor sketches and question marks. _Why, how, what._ She asked some of her questions out loud to Jack or her mother. “I wonder why…,” she would begin. A child with endless unanswerable questions would be a challenge to be around—a nightmare, actually. I moved away. On the third field trip, I was with about twenty-five people following Jack through a fern grove. The annoying girl with the endless questions was in front of me, and every thirty or forty feet, she stopped to examine whatever caught her eye. She turned over a fern frond and pointed to rows of golden brown dots. “Sporangia spores,” she said to her mother. “Fertile.” I looked. So that’s what those things on the back of ferns were, thousands of spores. She saw a manroot plant and traced it ten feet to where it started and ended. She saw birds in the distance. _Common Yellowthroat. Red-tailed Hawk, Ruby-crowned Kinglet._ I couldn’t find them. I blamed the floaters in my eyes. She cocked her ear toward a tree and listened. “Hermit Thrush. I love their song.” Her curiosity and exuberance over so many things brought me back to that time in my childhood when I crouched and touched plants and animals, when I turned things over to see what was underneath, when I happily spent hours lost in curiosity and exploration, and was never satiated. I may not have asked endless questions aloud, but as a kid in nature, I wondered about everything.

I think as written, this is one of the purest displays of “Learn and Be Curious” that you can find. For our purposes, the aftermath is maybe the interesting part: the girl from the passage later went on to become Tan’s mentor for nature journaling. What started as a batch of annoying questions became a knowledge base and later expertise.

When I was briefly experimenting with [Zig](https://ziglang.org/), there was one thing that stood out to me basically from the first program: printing to `stdout` requires you to handle a potential error value. At the time, I sort of knew in the back of my head that all I/O operations were always suspect, but it was the first time that I was forced to face this reality for something as simple as printing to the console. Normally, you just expect it to be infallible. But it turns out Zig got it right: [several widely-used languages have a bug in Hello World](https://blog.sunfishcode.online/bugs-in-hello-world/). The language asked me an annoying question, and in the process of answering it, I learned a bundle of things about computing and error handling.

Nowadays, if I leave a meeting worrying that I was too annoying with my questions, I tell myself that it means I’ve done something right.

### 3. A Tale of `FALSE`

For the past month and a half, my work has revolved around a new [Query Correctness Testing Framework](https://github.com/opensearch-project/sql/issues/3220). The premise essentially is to generate a large number of test cases to catch bugs that wouldn’t be practical to find with traditional unit testing by working backward from properties we expect queries to fulfill. As part of that, I’ve [skimmed Manuel Rigger’s 2020 preprint on Ternary Logic Partitioning](https://www.manuelrigger.at/preprints/TLP.pdf) a few times. The technical details aren’t too important, but at one part of the preprint they mention that for the initial implementation they only wrote a query generator that tests two features: integers and booleans.

Surprisingly, despite the remarkable simplicity of their system (they claim to have written it in two hours), they found bugs in MySQL, one of which is really fun:

```
CREATE TABLE t0(c0 INT);
INSERT INTO t0(c0) VALUES (0);
SELECT * FROM t0 WHERE 0.9 > t0.c0;
```

The `SELECT` statement should return one record as 0.9 > 0, but on the version tested, it returned no records because of a type conversion issue. I think if you asked just about anyone at the time if MySQL could have a bug so simple in `>`, the answer would be a resounding “no,” and yet here it was. The author expresses it quite nicely:

>This is one of multiple basic bugs that we found in MySQL. We still consider it interesting, since it shows that also mature DBMS are prone to such bugs.

At the time that I had read that particular passage, I was skeptical that our software would be similar. I was in the process of overengineering a query generator to directly target known bug reports, but I decided to go back to the basics and just try the same thing for the SQL plugin, integers, and booleans. To ask the easy questions.

Two days later, I’d found not one, not two, but nine different bugs around our boolean evaluation, across both PPL and SQL. Here are two of them: respectively, a failing PPL query and a failing SQL query, using a test index with one document. Both queries should return 0 records as their condition should never succeed.

```
SOURCE = test_index | WHERE FALSE
```

```
SELECT * FROM test_index WHERE FALSE
```

The first query crashes due to a syntax error; it turns out the ANTLR grammar for PPL didn’t allow putting boolean literals in WHERE clauses. Oops. The second query is marginally more subtle: it returns all documents from the index instead of zero because of an issue with constant expression optimization. Oops again.

In the process of designing an elaborate system for large-scale testing, I’d lost sight of the easy questions, like “What are some of the simplest queries that we can run?” Asking them uncovered a lot. I only wish I’d followed my own advice a month ago; I could probably have shaved a few hundred words off that RFC.
