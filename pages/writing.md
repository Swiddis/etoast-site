---
layout: base.njk
title: Writing
---

# {{ title }}

This is my little writing repository for blethering about miscellanious topics.
The documents are subject to receive updates, but changelogs will be maintained.

<ul>
{% for page in collections.writing %}
  <li>
    <a href="{{ page.url }}">{{ page.data.title }}</a> - {{ page.data.author_date }}
  </li>
{% endfor %}
</ul>
