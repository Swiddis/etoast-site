---
layout: base.njk
title: IDE-Driven Development
author_date: April 26, 2024
---

# {{ title }}

*What? Another one?*

There's a lot of them. [Test-Driven
Development](https://en.wikipedia.org/wiki/Test-driven_development), [Behavior-Driven
Development](https://en.wikipedia.org/wiki/Behavior-driven_development),
[Business-Driven](https://en.wikipedia.org/wiki/Business-driven_development),
[Model-Driven](https://en.wikipedia.org/wiki/Model-driven_engineering),
[Spec-Driven](https://blog.apideck.com/spec-driven-development-part-1),
[Security-Driven](https://www.oreilly.com/library/view/security-driven-software-development/9781835462836/),
[Documentation-Driven](https://gist.github.com/zsup/9434452),
[Readme-Driven](https://tom.preston-werner.com/2010/08/23/readme-driven-development.html)...
On my team we joke about Release-Driven Development, too. There's an abundance of
opinions on what should drive our development: anyone can take some aspect of software,
throw a "driven" on it, and call it a day. I realize I'm just making the situation
worse; that said, I've started to notice a pattern across several practices and pieces
of advice, and I'd like to call some attention to it, starting with a story.

Our story begins in the IDE. The main project I run at work involves, among other
things, a React frontend with Typescript. One day, with seemingly no changes, type
errors started appearing under many of our React components, which flooded the error
count for most files in the project. Doing some research on these particular errors, I
couldn't find the cause, and consulting my team didn't help either. Eventually I moved
on, ignoring the errors since they weren't otherwise affecting work. Soon enough I'd
internalized ignoring the errors, and forgotten about the issue entirely.

Fast forward some time later and for the first time I ran into [The Grug Brained
Developer](https://grugbrain.dev/). The whole article is a fun read, if not sometimes
tricky to parse, and generally centers around the idea that complexity is one of the
greatest evils in software. But when Grug discusses type systems, one part in particular
stuck out to me: the grug-brained developer thinks about type systems in terms of the
functionality it adds to their IDE.

> grug very like type systems make programming easier. for grug, type systems most value
> when grug hit dot on keyboard and list of things grug can do pop up magic. this 90% of
> value of type system or more to grug... danger abstraction too high, big brain type
> system code become astral projection of platonic generic turing model of computation
> into code base. grug confused and agree some level very elegant but also very hard do
> anything like record number of club inventory for Grug Inc. task at hand

According to Grug, type systems get in the way of work when they start requiring
constraints irrelevant to the task at hand. In contrast, they help most when they
enhance an editor's autocomplete and automatic error checking. This lines up with my own
experience: I add type hints to my projects in loosely-typed languages largely because
it takes away the cognitive load of remembering what methods need. When I thought about
Grug's statement in the context of my current project's flood of errors, I realized they
really *were* slowing me down, however slightly. I started missing other types of errors
more frequently, especially when refactoring, and spent more time verifying assumptions
that otherwise were machine-checked.

There's a book which, like Grug, also identifies complexity as one of the biggest
obstacles in software: John Ousterhout's *A Philosophy of Software Design*. This book is
[one of my favorites](/writing/programming-books), and in the section discussing the
idea that "good code is self-documenting," I want to draw attention to how Ousterhout
discusses commenting code in relation to a function's declaration:

> If users must read the code of a method in order to use it, then there is no
> abstraction: all of the complexity of the method is exposed. Without comments, the
> only abstraction of a method is its declaration, which specifies its name and the
> names and types of its arguments and results. The declaration is missing too much
> essential information to provide a useful abstraction by itself.

While more generally applicable to the nature of abstraction, I think this passage also
connects with the common flow of how we tend to work with code in practice. When using
libraries where it's available, I'm often hovering over different methods or classes to
get hints about how they work. If a method isn't documented, one is only armed with the
signature, and if I need more information I'll end up with another tab for the method's
source. In contrast to the mostly-harmless IDE errors above, a lack of documentation has
a more noticeable effect on development.

There's one more area I want to highlight here: testing. Having a live feedback loop
that tells you whether your code is working is invaluable for making dangerous changes.
This is one of the key aspects of Test-Driven Development, that getting a fast feedback
loop while writing the code saves time later. Pushing this idea has lead to some really
interesting development setups, both in terms of iteration speed[^1] and
correctness[^2].

So, what do these three areas tell us?

Developer tooling has been a hot topic since before we programmed with keyboards. A
large following of Vim and Emacs users dedicates hours to squeezing the last bits of
productivity out of their text editors, while tools like Copilot storm the industry with
AI-enriched autocomplete. In *The Pragmatic Programmer*, considerable emphasis is placed
on learning your tools well, and it's common advice to get well-acquainted with a
debugger. These discussions won't stop anytime soon (nor should they).

I think that taking the time to invest in finding good tools and actually learning how
to use them is a relatively low-effort way to hit a lot of related good practices, this
is what I mean by IDE-Driven Development[^3]. Doing this implies a certain attitude
about continually learning your tools, getting familiar with the shell, learning your
build system, learning a debugger, and automating tedious tasks. Type safety and
code-level documentation make it possible to infer correct method usage without leaving
your current tab, while thorough unit tests allow for quick and fearless revision.
Making good configuration and setup guides for your software speeds up onboarding,
identified as a high-leverage activity by *The Effective Engineer* author Edmond Lau.

Let's not forget the opposite direction, too: when software is difficult to build, test,
or configure, developers struggle to get up to speed and have low confidence in the
results. I've wasted many hours struggling to work on software where I just couldn't get
the thing to actually build, and many more re-treading beaten paths in hacking together
shell scripts to cover configuration deficiencies. Forgetting to work towards a
reproducible development environment leads to "works on my machine"-ness and hurts
efficiency over time.

Good developer experience contributes to better software. If you let your developer
environment help drive you, then you'll develop a toolkit for continually enhancing the
experience of yourself and others.

[^1]: A friend recently directed me to the [Tomorrow Corporation Tech Demo](https://www.youtube.com/watch?v=72y2EC5fkcE) which shows really stunning live debugging capabilities.
[^2]: While reading about [how AWS S3 achieves eleven 9s of durability](https://highscalability.com/behind-aws-s3s-massive-scale/) it struck me just how much durability was a part of their process, including putting formal verification in their unit tests.
[^3]: I'm using the term IDE fairly loosely, not as one specific IDE software but as the wider system one uses to write code on their machine. I should probably call this DE-Driven Development, but IDE is a more generally recognizable term, and linguistically IDE seems to already be drifting away from meaning specific software in phrases like "Get out of the IDE".
