// Copyright 2015 Google Inc. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
#![allow(unused_variables, dead_code)]
//! HTML renderer that takes an iterator of events as input.

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;
use std::marker::PhantomData;

use parse::{Event, Tag};
use parse::Event::{Start, End, Text, Html, InlineHtml, SoftBreak, HardBreak, FootnoteReference};
use parse::Alignment;
use escape::{escape_html, escape_href};

pub trait IntoHtml<C> {
    fn render(self, ctx: &mut C, buf: &mut String);
}

enum TableState {
    Head,
    Body,
}

pub fn fresh_line(buf: &mut String) {
    if !(buf.is_empty() || buf.ends_with('\n')) {
        buf.push('\n');
    }
}

fn start_tag<'a>(buf: &mut String, tag: Tag<'a>) {
    buf.push_str("<pre><code>");
}

fn end_tag<'a>(buf: &mut String, tag: Tag<'a>) {
    buf.push_str("</code></pre>\n");
}

impl<'a> IntoHtml<Context<'a>> for Event<'a> {
    fn render(self, ctx: &mut Context<'a>, buf: &mut String) {
        match self {
            Start(tag) => start_tag(buf, tag),
            End(tag) => end_tag(buf, tag),
            Text(text) => escape_html(buf, &text, false),
            // Html(html) |
            // InlineHtml(html) => buf.push_str(&html),
            // SoftBreak => buf.push('\n'),
            // HardBreak => buf.push_str("<br />\n"),
            // FootnoteReference(name) => {
            //     let len = ctx.numbers.len() + 1;
            //     buf.push_str("<sup class=\"footnote-reference\"><a href=\"#");
            //     escape_html(buf, &*name, false);
            //     buf.push_str("\">");
            //     let number = ctx.numbers.entry(name).or_insert(len);
            //     buf.push_str(&*format!("{}", number));
            //     buf.push_str("</a></sup>");
            // },
            _ => (),
        }
    }
}

pub struct Context<'a> {
    numbers: HashMap<Cow<'a, str>, usize>,
    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
}

impl<'a> Context<'a> {
    fn new() -> Context<'a> {
        Context {
            numbers: HashMap::new(),
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
        }
    }
}

pub struct Renderer<'a, T, I>
where
    T: 'a + IntoHtml<Context<'a>>,
    I: Iterator<Item = T>
{
    events: I,
    _t: PhantomData<&'a T>,
}

impl<'a, T ,I> Renderer<'a, T, I>
where
    T: IntoHtml<Context<'a>>,
    I: Iterator<Item = T>
{
    pub fn new(events: I) -> Renderer<'a, T, I> {
        Renderer {
            events,
            _t : PhantomData
        }
    }

    pub fn render(mut self, buf: &mut String) {
        let mut ctx = Context::new();
        while let Some(event) = self.events.next() {
            event.render(&mut ctx, buf)
        }
    }
}

struct Ctx<'a, 'b, I: IntoIterator> {
    iter: I,
    buf: &'b mut String,
    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    numbers: HashMap<Cow<'a, str>, usize>,
}

impl<'a, 'b, I: Iterator<Item = Event<'a>>> Ctx<'a, 'b, I> {
    fn fresh_line(&mut self) {
        if !(self.buf.is_empty() || self.buf.ends_with('\n')) {
            self.buf.push('\n');
        }
    }

    pub fn run(&mut self) {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => self.start_tag(tag),
                End(tag) => self.end_tag(tag),
                Text(text) => escape_html(self.buf, &text, false),
                Html(html) |
                InlineHtml(html) => self.buf.push_str(&html),
                SoftBreak => self.buf.push('\n'),
                HardBreak => self.buf.push_str("<br />\n"),
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    self.buf.push_str("<sup class=\"footnote-reference\"><a href=\"#");
                    escape_html(self.buf, &*name, false);
                    self.buf.push_str("\">");
                    let number = self.numbers.entry(name).or_insert(len);
                    self.buf.push_str(&*format!("{}", number));
                    self.buf.push_str("</a></sup>");
                },
            }
        }
    }

    fn start_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::Paragraph => {
                self.fresh_line();
                self.buf.push_str("<p>");
            }
            Tag::Rule => {
                self.fresh_line();
                self.buf.push_str("<hr />\n")
            }
            Tag::Header(level) => {
                self.fresh_line();
                self.buf.push_str("<h");
                self.buf.push((b'0' + level as u8) as char);
                self.buf.push('>');
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;
                self.buf.push_str("<table>");
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.buf.push_str("<thead><tr>");
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                self.buf.push_str("<tr>");
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => self.buf.push_str("<th"),
                    TableState::Body => self.buf.push_str("<td"),
                }
                match self.table_alignments.get(self.table_cell_index) {
                    Some(&Alignment::Left) => self.buf.push_str(" align=\"left\""),
                    Some(&Alignment::Center) => self.buf.push_str(" align=\"center\""),
                    Some(&Alignment::Right) => self.buf.push_str(" align=\"right\""),
                    _ => (),
                }
                self.buf.push_str(">");
            }
            Tag::BlockQuote => {
                self.fresh_line();
                self.buf.push_str("<blockquote>\n");
            }
            Tag::CodeBlock(info) => {
                self.fresh_line();
                let lang = info.split(' ').next().unwrap();
                if lang.is_empty() {
                    self.buf.push_str("<pre><code>");
                } else {
                    self.buf.push_str("<pre><code class=\"language-");
                    escape_html(self.buf, lang, false);
                    self.buf.push_str("\">");
                }
            }
            Tag::List(Some(1)) => {
                self.fresh_line();
                self.buf.push_str("<ol>\n");
            }
            Tag::List(Some(start)) => {
                self.fresh_line();
                let _ = write!(self.buf, "<ol start=\"{}\">\n", start);
            }
            Tag::List(None) => {
                self.fresh_line();
                self.buf.push_str("<ul>\n");
            }
            Tag::Item => {
                self.fresh_line();
                self.buf.push_str("<li>");
            }
            Tag::Emphasis => self.buf.push_str("<em>"),
            Tag::Strong => self.buf.push_str("<strong>"),
            Tag::Code => self.buf.push_str("<code>"),
            Tag::Link(dest, title) => {
                self.buf.push_str("<a href=\"");
                escape_href(self.buf, &dest);
                if !title.is_empty() {
                    self.buf.push_str("\" title=\"");
                    escape_html(self.buf, &title, false);
                }
                self.buf.push_str("\">");
            }
            Tag::Image(dest, title) => {
                self.buf.push_str("<img src=\"");
                escape_href(self.buf, &dest);
                self.buf.push_str("\" alt=\"");
                self.raw_text();
                if !title.is_empty() {
                    self.buf.push_str("\" title=\"");
                    escape_html(self.buf, &title, false);
                }
                self.buf.push_str("\" />")
            }
            Tag::FootnoteDefinition(name) => {
                self.fresh_line();
                let len = self.numbers.len() + 1;
                self.buf.push_str(
                    "<div class=\"footnote-definition\" id=\"",
                );
                escape_html(self.buf, &*name, false);
                self.buf.push_str(
                    "\"><sup class=\"footnote-definition-label\">",
                );
                let number = self.numbers.entry(name).or_insert(len);
                self.buf.push_str(&*format!("{}", number));
                self.buf.push_str("</sup>");
            }
        }
    }

    fn end_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => self.buf.push_str("</p>\n"),
            Tag::Rule => (),
            Tag::Header(level) => {
                self.buf.push_str("</h");
                self.buf.push((b'0' + level as u8) as char);
                self.buf.push_str(">\n");
            }
            Tag::Table(_) => {
                self.buf.push_str("</tbody></table>\n");
            }
            Tag::TableHead => {
                self.buf.push_str("</tr></thead><tbody>\n");
                self.table_state = TableState::Body;
            }
            Tag::TableRow => {
                self.buf.push_str("</tr>\n");
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => self.buf.push_str("</th>"),
                    TableState::Body => self.buf.push_str("</td>"),
                }
                self.table_cell_index += 1;
            }
            Tag::BlockQuote => self.buf.push_str("</blockquote>\n"),
            Tag::CodeBlock(_) => self.buf.push_str("</code></pre>\n"),
            Tag::List(Some(_)) => self.buf.push_str("</ol>\n"),
            Tag::List(None) => self.buf.push_str("</ul>\n"),
            Tag::Item => self.buf.push_str("</li>\n"),
            Tag::Emphasis => self.buf.push_str("</em>"),
            Tag::Strong => self.buf.push_str("</strong>"),
            Tag::Code => self.buf.push_str("</code>"),
            Tag::Link(_, _) => self.buf.push_str("</a>"),
            Tag::Image(_, _) => (), // shouldn't happen, handled in start
            Tag::FootnoteDefinition(_) => self.buf.push_str("</div>\n"),
        }
    }

    // run raw text, consuming end tag
    fn raw_text<'c>(&mut self) {
        let mut nest = 0;
        while let Some(event) = self.iter.next() {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Text(text) => escape_html(self.buf, &text, false),
                Html(_) => (),
                InlineHtml(html) => escape_html(self.buf, &html, false),
                SoftBreak | HardBreak => self.buf.push(' '),
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let number = self.numbers.entry(name).or_insert(len);
                    self.buf.push_str(&*format!("[{}]", number));
                }
            }
        }
    }
}

/// Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
/// push it to a `String`.
///
/// # Examples
///
/// ```
/// use pulldown_cmark::{html, Parser};
///
/// let markdown_str = r#"
/// hello
/// =====
///
/// * alpha
/// * beta
/// "#;
/// let parser = Parser::new(markdown_str);
///
/// let mut html_buf = String::new();
/// html::push_html(&mut html_buf, parser);
///
/// assert_eq!(html_buf, r#"<h1>hello</h1>
/// <ul>
/// <li>alpha</li>
/// <li>beta</li>
/// </ul>
/// "#);
/// ```
pub fn push_html<'a, I: Iterator<Item = Event<'a>>>(buf: &mut String, iter: I) {
    let renderer = Renderer::new(iter);
    renderer.render(buf);
}
