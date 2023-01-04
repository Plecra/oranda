use std::path::Path;

use axohtml::{dom::DOMTree, html, text, unsafe_text};

use crate::config::logo;
use crate::config::{theme, Config};
use axohtml::elements::{div, header, li, meta};

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
fn create_social_cards(config: &Config) -> Vec<Box<meta<String>>> {
    let mut html = vec![];
    match config.social.as_ref() {
        Some(social) => {
            if let Some(image) = social.image.as_ref() {
                html.extend(html!(<meta name="twitter:card" content="summary_large_image"/>));

                html.extend(html!(<meta property="og:image" content=image />));
            };
            if let Some(image_alt) = social.image_alt.as_ref() {
                html.extend(html!(<meta property="og:image:alt" content=image_alt />));
            }

            if let Some(twitter_account) = social.twitter_account.as_ref() {
                html.extend(html!(<meta name="twitter:creator" content=twitter_account/>));
                html.extend(html!(<meta name="twitter:site" content=twitter_account/>));
            };

            Some(())
        }

        None => None,
    };

    html
}

pub fn build(config: &Config, content: String) -> String {
    let theme = theme::css_class(&config.theme);
    let classlist: &str = &format!("body {}", theme)[..];
    let description = &config.description;
    let header = create_header(config);
    let homepage = config.homepage.as_ref().map(|homepage| {
        html!(
          <meta property="og:url" content=homepage/>
        )
    });
    let social_meta = create_social_cards(config);
    let banner = repo_banner(config);
    let doc: DOMTree<String> = html!(
    <html lang="en" id="oranda">
    <head>
    <title>{ text!(&config.name) }</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    { homepage }
    <meta name="description" content=description />
    <meta property="og:description" content=description/>
    <meta property="og:type" content="website" />
    <meta property="og:title" content=&config.name />
    {social_meta}
    <link rel="stylesheet" href="styles.css"></link>
    </head>
    <body>
    <div class=classlist>
        {banner}

        <div class="container">{header}{ unsafe_text!(content) }</div>
    </div>
    </body>
    </html>
         );
    doc.to_string()
}

fn repo_banner(config: &Config) -> Option<Box<div<String>>> {
    config.repository.as_ref().map(|repository| {
        html!(
        <div class="repo_banner">
            <a href=repository>
                <div class="icon" aria-hidden="true"/>
                {text!("Check out our GitHub")}
            </a>
        </div>
                )
    })
}

fn create_header(config: &Config) -> Option<Box<header<String>>> {
    if config.no_header {
        return None;
    }
    let logo = logo::get_logo(config);
    let nav = match config.additional_pages.as_ref() {
        Some(pages) => {
            let mut html: Vec<Box<li<String>>> = vec![html!(<li><a href="/">"Home"</a></li>)];
            for page in pages.iter() {
                let path = Path::new(page);
                let file_name = path
                    .file_stem()
                    .unwrap_or(path.as_os_str())
                    .to_string_lossy();
                let path = format!("/{}", file_name);
                html.extend(html!(<li><a href=path>{text!(file_name)}</a></li>));
            }
            Some(html!(
            <nav>
                <ul>
                     {html}
                </ul>
            </nav>
            ))
        }
        None => None,
    };

    Some(html!(
        <header>
            {nav}
            <h1>{text!(&config.name)}</h1>
            {logo.unwrap()}
        </header>
    ))
}
