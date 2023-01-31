use std::path::Path;

pub mod artifacts;
mod layout;
pub mod markdown;
pub mod page;
use page::Page;

use crate::config::Config;
use crate::errors::*;

#[derive(Debug)]
pub struct Site {
    pages: Vec<Page>,
}

impl Site {
    pub fn build(config: &Config) -> Result<Site> {
        let index = Page::new_from_file(config, &config.readme_path)?;
        let mut pages = vec![index];
        if let Some(files) = &config.additional_pages {
            for file in files {
                let additional_page = Page::new_from_file(config, file)?;
                pages.push(additional_page)
            }
        }
        if config.artifacts.is_some() {
            let artifacts_html = artifacts::page::build(config)?;
            let artifacts_page = Page::new_from_contents(artifacts_html, "artifacts.html");
            pages.push(artifacts_page)
        }

        Ok(Site { pages })
    }

    pub fn copy_static(dist_path: &String, static_path: &String) -> Result<()> {
        Self::create_dist_dir(dist_path)?;
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        fs_extra::copy_items(&[static_path], dist_path, &options)?;

        Ok(())
    }

    pub fn write(self, config: &Config) -> Result<()> {
        let dist = &config.dist_dir;
        Self::create_dist_dir(dist)?;
        for page in self.pages {
            let asset = axoasset::local::LocalAsset::new(
                &page.filename.clone(),
                page.build(config)?.into(),
            );
            axoasset::local::LocalAsset::write(&asset, dist)?;
        }
        if let Some(book_path) = &config.md_book {
            Self::copy_static(dist, book_path)?;
        }
        if Path::new(&config.static_dir).exists() {
            Self::copy_static(dist, &config.static_dir)?;
        }

        Ok(())
    }

    fn create_dist_dir(dist_path: &String) -> Result<()> {
        if !Path::new(dist_path).exists() {
            std::fs::create_dir_all(dist_path)?;
        }

        Ok(())
    }
}
