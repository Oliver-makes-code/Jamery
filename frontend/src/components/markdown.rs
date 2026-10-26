use leptos::prelude::*;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::components::block::Block;

fn markdown_options() -> Options {
    return Options::ENABLE_TABLES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_FOOTNOTES;
}

fn safe_url(url: &str) -> Option<String> {
    let url = url.trim();

    if url.is_empty() {
        return None;
    }

    let lower = url.to_ascii_lowercase();

    if lower.starts_with("https://")
        || lower.starts_with("http://")
        || lower.starts_with("mailto:")
        || !url.contains(':')
    {
        return Some(url.to_string());
    }

    return None;
}

fn footnote_id(label: &str) -> String {
    let mut id = String::from("fn-");

    for character in label.chars() {
        if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
            id.push(character);
        } else if character.is_whitespace() {
            id.push('-');
        }
    }

    return id;
}

struct ViewFrame<'a> {
    parent: Vec<AnyView>,
    tag: Tag<'a>,
    image_alt: String,
}

fn render_tag<'a>(
    tag: Tag<'a>,
    children: Vec<AnyView>,
    image_alt: String,
    in_table_head: bool,
) -> Option<AnyView> {
    match tag {
        Tag::Paragraph => {
            return Some(view! { <p>{children}</p> }.into_any());
        }

        Tag::Heading {
            level, id, classes, ..
        } => {
            let id = id.map(|value| value.to_string());

            let class = classes
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            match level {
                HeadingLevel::H1 => {
                    return Some(
                        view! {
                            <h1 id=id class=class>{children}</h1>
                        }
                        .into_any(),
                    );
                }
                HeadingLevel::H2 => {
                    return Some(
                        view! {
                            <h2 id=id class=class>{children}</h2>
                        }
                        .into_any(),
                    );
                }
                HeadingLevel::H3 => {
                    return Some(
                        view! {
                            <h3 id=id class=class>{children}</h3>
                        }
                        .into_any(),
                    );
                }
                HeadingLevel::H4 => {
                    return Some(
                        view! {
                            <h4 id=id class=class>{children}</h4>
                        }
                        .into_any(),
                    );
                }
                HeadingLevel::H5 => {
                    return Some(
                        view! {
                            <h5 id=id class=class>{children}</h5>
                        }
                        .into_any(),
                    );
                }
                HeadingLevel::H6 => {
                    return Some(
                        view! {
                            <h6 id=id class=class>{children}</h6>
                        }
                        .into_any(),
                    );
                }
            }
        }

        Tag::BlockQuote(_) => {
            return Some(
                view! {
                    <blockquote>{children}</blockquote>
                }
                .into_any(),
            );
        }

        Tag::CodeBlock(kind) => match kind {
            CodeBlockKind::Indented => {
                return Some(
                    view! {
                        <pre><code>{children}</code></pre>
                    }
                    .into_any(),
                );
            }

            CodeBlockKind::Fenced(info) => {
                let language = info.split_whitespace().next().unwrap_or("").to_string();

                let class = if language.is_empty() {
                    String::new()
                } else {
                    format!("language-{language}")
                };

                return Some(
                    view! {
                        <pre><code class=class>{children}</code></pre>
                    }
                    .into_any(),
                );
            }
        },

        Tag::List(Some(start)) => {
            return Some(
                view! {
                    <ol start=start>{children}</ol>
                }
                .into_any(),
            );
        }

        Tag::List(None) => {
            return Some(
                view! {
                    <ul>{children}</ul>
                }
                .into_any(),
            );
        }

        Tag::Item => {
            return Some(view! { <li>{children}</li> }.into_any());
        }

        Tag::Emphasis => {
            return Some(view! { <em>{children}</em> }.into_any());
        }

        Tag::Strong => {
            return Some(view! { <strong>{children}</strong> }.into_any());
        }

        Tag::Strikethrough => {
            return Some(view! { <del>{children}</del> }.into_any());
        }

        Tag::Superscript => {
            return Some(view! { <sup>{children}</sup> }.into_any());
        }

        Tag::Subscript => {
            return Some(view! { <sub>{children}</sub> }.into_any());
        }

        Tag::Link {
            dest_url, title, ..
        } => {
            let Some(href) = safe_url(&dest_url) else {
                return Some(view! { <>{children}</> }.into_any());
            };

            let title = title.to_string();

            if title.is_empty() {
                return Some(
                    view! {
                        <a href=href>{children}</a>
                    }
                    .into_any(),
                );
            }

            return Some(
                view! {
                    <a href=href title=title>{children}</a>
                }
                .into_any(),
            );
        }

        Tag::Image {
            dest_url, title, ..
        } => {
            let Some(src) = safe_url(&dest_url) else {
                return Some(
                    view! {
                        <span>{image_alt}</span>
                    }
                    .into_any(),
                );
            };

            let title = title.to_string();

            if title.is_empty() {
                return Some(
                    view! {
                        <img src=src alt=image_alt />
                    }
                    .into_any(),
                );
            }

            return Some(
                view! {
                    <img src=src alt=image_alt title=title />
                }
                .into_any(),
            );
        }

        Tag::Table(_) => {
            return Some(
                view! {
                    <div class="table-wrapper">
                        <table>{children}</table>
                    </div>
                }
                .into_any(),
            );
        }

        Tag::TableHead => {
            return Some(
                view! {
                    <thead>{children}</thead>
                }
                .into_any(),
            );
        }

        Tag::TableRow => {
            return Some(
                view! {
                    <tr>{children}</tr>
                }
                .into_any(),
            );
        }

        Tag::TableCell => {
            if in_table_head {
                return Some(
                    view! {
                        <th>{children}</th>
                    }
                    .into_any(),
                );
            }

            return Some(
                view! {
                    <td>{children}</td>
                }
                .into_any(),
            );
        }

        Tag::FootnoteDefinition(label) => {
            let id = footnote_id(&label);

            return Some(
                view! {
                    <div class="my-2" id=id>
                        {children}
                    </div>
                }
                .into_any(),
            );
        }

        // No HTML.
        Tag::HtmlBlock | Tag::MetadataBlock(_) => {
            return None;
        }

        _ => {
            return Some(view! { <>{children}</> }.into_any());
        }
    }
}

fn render_markdown(markdown: &str) -> Vec<AnyView> {
    let parser = Parser::new_ext(markdown, markdown_options());

    let mut view_stack: Vec<ViewFrame> = Vec::new();
    let mut current_view: Vec<AnyView> = Vec::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                view_stack.push(ViewFrame {
                    parent: std::mem::take(&mut current_view),
                    tag,
                    image_alt: String::new(),
                });
            }

            Event::End(tag_end) => {
                let Some(frame) = view_stack.pop() else {
                    continue;
                };

                if !tag_end_matches(&frame.tag, &tag_end) {
                    view_stack.push(frame);
                    continue;
                }

                let children = std::mem::take(&mut current_view);

                let in_table_head = view_stack
                    .iter()
                    .any(|frame| matches!(&frame.tag, Tag::TableHead));

                current_view = frame.parent;

                if let Some(view) = render_tag(frame.tag, children, frame.image_alt, in_table_head)
                {
                    current_view.push(view);
                }
            }

            Event::Text(text) => {
                if let Some(frame) = view_stack
                    .iter_mut()
                    .rev()
                    .find(|frame| matches!(&frame.tag, Tag::Image { .. }))
                {
                    frame.image_alt.push_str(&text);
                }

                current_view.push(text.to_string().into_any());
            }

            Event::Code(code) => {
                current_view
                    .push(view! { <code class="inline-code">{code.to_string()}</code> }.into_any());
            }

            Event::InlineMath(math) => {
                current_view.push(
                    view! {
                        <span class="font-mono">{math.to_string()}</span>
                    }
                    .into_any(),
                );
            }

            Event::DisplayMath(math) => {
                current_view.push(
                    view! {
                        <span class="font-mono">{math.to_string()}</span>
                    }
                    .into_any(),
                );
            }

            Event::SoftBreak => {
                current_view.push(" ".to_string().into_any());
            }

            Event::HardBreak => {
                current_view.push(view! { <br /> }.into_any());
            }

            Event::Rule => {
                current_view.push(view! { <hr /> }.into_any());
            }

            Event::TaskListMarker(checked) => {
                current_view.push(
                    view! {
                        <input
                            type="checkbox"
                            checked=checked
                            disabled
                        />
                    }
                    .into_any(),
                );
            }

            Event::FootnoteReference(label) => {
                let label = label.to_string();
                let href = format!("#{}", footnote_id(&label));

                current_view.push(
                    view! {
                        <sup>
                            <a href=href>{label}</a>
                        </sup>
                    }
                    .into_any(),
                );
            }

            // No HTML.
            Event::Html(_) | Event::InlineHtml(_) => {}
        }
    }

    return current_view;
}

fn tag_end_matches(tag: &Tag<'_>, end: &TagEnd) -> bool {
    match (tag, end) {
        (Tag::Paragraph, TagEnd::Paragraph) => return true,

        (Tag::Heading { level, .. }, TagEnd::Heading(end_level)) => {
            return level == end_level;
        }

        (Tag::BlockQuote(_), TagEnd::BlockQuote(_)) => return true,

        (Tag::CodeBlock(_), TagEnd::CodeBlock) => return true,

        (Tag::List(_), TagEnd::List(_)) => return true,

        (Tag::Item, TagEnd::Item) => return true,

        (Tag::FootnoteDefinition(_), TagEnd::FootnoteDefinition) => return true,

        (Tag::Table(_), TagEnd::Table) => return true,

        (Tag::TableHead, TagEnd::TableHead) => return true,

        (Tag::TableRow, TagEnd::TableRow) => return true,

        (Tag::TableCell, TagEnd::TableCell) => return true,

        (Tag::Emphasis, TagEnd::Emphasis) => return true,

        (Tag::Strong, TagEnd::Strong) => return true,

        (Tag::Strikethrough, TagEnd::Strikethrough) => return true,

        (Tag::Superscript, TagEnd::Superscript) => return true,

        (Tag::Subscript, TagEnd::Subscript) => return true,

        (Tag::Link { .. }, TagEnd::Link) => return true,

        (Tag::Image { .. }, TagEnd::Image) => return true,

        (Tag::DefinitionList, TagEnd::DefinitionList) => return true,

        (Tag::DefinitionListTitle, TagEnd::DefinitionListTitle) => return true,

        (Tag::DefinitionListDefinition, TagEnd::DefinitionListDefinition) => return true,

        (Tag::HtmlBlock, TagEnd::HtmlBlock) => return true,

        (Tag::MetadataBlock(_), TagEnd::MetadataBlock(_)) => return true,

        _ => return false,
    }
}

#[component]
pub fn Markdown(markdown: String) -> impl IntoView {
    let children = render_markdown(&markdown);

    return view! {
        {children}
    };
}

#[component]
pub fn MarkdownBlock(markdown: String) -> impl IntoView {
    return view! {
        <Block><Markdown markdown=markdown.to_owned() /></Block>
    };
}
