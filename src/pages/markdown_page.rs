use crate::parser::ParseAct;
use crate::ParseActContext;
use crate::{parser::str2blog, Blog};
use crate::pages::post::{read_file, FetchError, FetchState};
use yew::{html, prelude::*, Component, Context, Html};
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub filename: String,
    pub default_title: String,
}

pub struct MarkdownPage {
    state: FetchState<Blog>,
}

pub enum Msg {
    SetBlogFetchState(FetchState<Blog>),
}

impl Component for MarkdownPage {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (parser, _) = ctx
            .link()
            .context::<ParseActContext>(Callback::noop())
            .expect("Parser Context not found");
        
        let filename = ctx.props().filename.clone();
        let default_title = ctx.props().default_title.clone();
        
        ctx.link().send_future(async move {
            let path = format!("posts/{}.rmd", filename);
            match read_file(&path).await {
                Ok(res) => {
                    log::debug!("Loaded markdown file: {}", path);
                    // Create a simple BlogMeta for this markdown file
                    let meta = crate::content::BlogMeta {
                        id: 0,
                        title: default_title,
                        timestamp: 0,
                        date: String::new(),
                        path: std::path::PathBuf::from(path.clone()),
                        hero: String::new(),
                    };
                    match str2blog(&res, &meta) {
                        Some(blog) => Msg::SetBlogFetchState(FetchState::Success(blog)),
                        None => Msg::SetBlogFetchState(FetchState::Failed(FetchError {
                            err: "Failed to parse markdown".into(),
                        })),
                    }
                }
                Err(err) => {
                    log::error!("Failed to load {}: {:?}", path, err);
                    Msg::SetBlogFetchState(FetchState::Failed(err))
                }
            }
        });
        Self {
            state: FetchState::Fetching,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetBlogFetchState(fetch_state) => {
                self.state = fetch_state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            FetchState::NotFetching => {
                html! {
                    <div class="markdown-page">
                        <div class="container">
                            <p>{ "Not fetching..." }</p>
                        </div>
                    </div>
                }
            }
            FetchState::Fetching => {
                html! {
                    <div class="markdown-page">
                        <div class="container">
                            <p>{ "Loading..." }</p>
                        </div>
                    </div>
                }
            }
            FetchState::Success(blog) => {
                let content_html = self.render_content(blog);
                html! {
                    <main class="markdown-page">
                        <div class="container">
                            <article class="markdown-content">
                                <h1>{ &blog.meta.title }</h1>
                                { content_html }
                            </article>
                        </div>
                    </main>
                }
            }
            FetchState::Failed(_) => {
                html! {
                    <main class="markdown-page">
                        <div class="container">
                            <h1>{ &ctx.props().default_title }</h1>
                            <p>{ "Failed to load page content. Please try again later." }</p>
                        </div>
                    </main>
                }
            }
        }
    }
}

impl MarkdownPage {
    fn render_content(&self, blog: &Blog) -> Html {
        let dom_parser = web_sys::DomParser::new().unwrap();
        let mut parts = Vec::new();
        
        use pulldown_cmark::Options;
        let mut options = Options::empty();
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        
        blog.content.iter().for_each(|part| {
            let parser = pulldown_cmark::Parser::new_ext(&part, options);
            let mut output = String::with_capacity(part.len() * 3 / 2);
            pulldown_cmark::html::push_html(&mut output, parser);
            let output_div = format!("<div> {} </div>", output);
            
            let parse_result = dom_parser
                .parse_from_string(&output_div, web_sys::SupportedType::TextHtml)
                .expect("Failed to parse as html");
            
            let body = parse_result.body().expect("The html had no body");
            let children = body.children();
            for i in 0..children.length() {
                let item = children.item(i).unwrap();
                let node = web_sys::Node::from(item);
                parts.push(Html::VRef(node.into()));
            }
        });
        
        html! { for parts }
    }
}
