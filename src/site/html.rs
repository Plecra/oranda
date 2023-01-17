use crate::config::analytics::{get_analytics, Analytics};
use crate::config::{theme, Config};
use axohtml::elements::{div, meta};

use crate::site::header;

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

use axohtml::{dom::DOMTree, html, text, unsafe_text};

pub fn build(config: &Config, content: String) -> String {
    let theme = theme::css_class(&config.theme);
    let analytics = get_analytics(config);
    let google_script = match &config.analytics {
        Some(Analytics::Google(g)) => Some(g.get_script()),
        _ => None,
    };
    let description = &config.description;
    let header = match config.no_header {
        true => None,
        false => Some(header::create(config)),
    };
    let homepage = config.homepage.as_ref().map(|homepage| {
        html!(
          <meta property="og:url" content=homepage/>
        )
    });
    let social_meta = create_social_cards(config);
    let banner = repo_banner(config);

    let doc: DOMTree<String> = html!(
    <html lang="en" id="oranda" class=theme>
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
    <link rel="stylesheet" href="https://www.unpkg.com/@axodotdev/fringe/themes/axo-oranda.css"></link>
    <link rel="stylesheet" href="styles.css"></link>
    </head>
    <body>
    <div class="container">
        {banner}
        <main>{header}{ unsafe_text!(content) }</main>
    </div>
        {analytics}
        {google_script}
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
