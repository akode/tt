---
title: "{{ title }}"
authors:
{%- for author in authors %}
  - {{ author -}}
{% endfor %}
annotation-target: {{ pdf_path }}
{%- match year %}
{% when Some with (val) %}
year: {{ val }}
{% when None %}
{% endmatch -%}
link: {{ pdf_link }}
---

## Abstract
{{ summary }}
