//! This module provides a pagination component for a web application using the Yew framework.
//! The component allows for navigation between multiple pages of content, displaying a series of
//! links to different pages and providing navigation buttons for moving forward and backward through
//! the pages.
//!
//! The component uses the `serde` crate for serialization and deserialization of query parameters,
//! and the `yew` and `yew_router` crates for building the web interface and handling routing.

use serde::Deserialize;
use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

const ELLIPSIS: &str = "\u{02026}";

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PageQuery {
    pub page: u64,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub page: u64,
    pub total_pages: u64,
    pub route_to_page: Route,
}

pub struct Pagination;

impl Component for Pagination {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::debug!(
            "page: {}, total: {}",
            ctx.props().page,
            ctx.props().total_pages
        );
        if ctx.props().page == 0 {
            log::debug!("no posts to pagination");
            html! { {"No Page Found"} }
        } else {
            html! {
                <>
                <nav class="pagination is-centered" role="navigation" aria-label="pagination">
                    { self.view_relnav_buttons(ctx.props()) }
                    <ul class="pagination-list">
                        { self.view_links(ctx.props()) }
                    </ul>
                </nav>
                </>
            }
        }
    }
}
impl Pagination {
    fn render_link(&self, to_page: u64, props: &Props) -> Html {
        let Props {
            page,
            route_to_page,
            ..
        } = props.clone();

        let is_current_class = if to_page == page { "is-current" } else { "" };

        html! {
            <li>
                <Link<Route, PageQuery>
                    classes={classes!("pagination-link", is_current_class)}
                    to={route_to_page}
                    query={Some(PageQuery{page: to_page})}
                >
                    { to_page }
                </Link<Route, PageQuery>>
            </li>
        }
    }

    fn render_links<P>(&self, mut pages: P, len: usize, max_links: usize, props: &Props) -> Html
    where
        P: Iterator<Item = u64> + DoubleEndedIterator,
    {
        if len > max_links {
            let last_link = self.render_link(pages.next_back().unwrap(), props);
            // remove 1 for the ellipsis and 1 for the last link
            let links = pages
                .take(max_links - 2)
                .map(|page| self.render_link(page, props));
            html! {
                <>
                    { for links }
                    <li><span class="pagination-ellipsis">{ ELLIPSIS }</span></li>
                    { last_link }
                </>
            }
        } else {
            html! { for pages.map(|page| self.render_link(page, props)) }
        }
    }

    fn view_links(&self, props: &Props) -> Html {
        const LINKS_PER_SIDE: usize = 3;

        let Props {
            page, total_pages, ..
        } = *props;

        let pages_prev = page.checked_sub(1).unwrap_or_default() as usize;
        let pages_next = (total_pages - page) as usize;

        let links_left = LINKS_PER_SIDE.min(pages_prev)
            // if there are less than `LINKS_PER_SIDE` to the right, we add some more on the left.
            + LINKS_PER_SIDE.checked_sub(pages_next).unwrap_or_default();
        let links_right = 2 * LINKS_PER_SIDE - links_left;

        html! {
 
        }
    }

    fn view_relnav_buttons(&self, props: &Props) -> Html {
        let Props {
            page,
            total_pages,
            route_to_page: to,
        } = props.clone();

        html! {
    
        }
    }
}
