# Adding Markdown Pages

This guide explains how to add new standalone pages using markdown files, similar to the Privacy and Terms pages.

## Quick Start

### 1. Create Your Markdown File

Create a new `.rmd` file in the `posts/` directory:

```bash
touch posts/about.rmd
```

### 2. Add Content

Write your markdown content in the file:

```markdown
# About Us

This is the about page content written in markdown.

## Our Mission

We provide excellent legal services...

## Contact Information

- Email: contact@example.com
- Phone: (555) 123-4567
```

### 3. Add a Route

Edit `src/main.rs` to add your new route:

1. Add your route to the `Route` enum:

```rust
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/privacy")]
    Privacy,
    #[at("/terms")]
    Terms,
    #[at("/about")]  // Add this
    About,           // Add this
    #[at("/posts/:id/:title")]
    Post { id: u32, title: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
```

2. Add a case in the `switch` function:

```rust
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage /> },
        Route::Privacy => html! {
            <MarkdownPage filename="privacy" default_title="Privacy Policy" />
        },
        Route::Terms => html! {
            <MarkdownPage filename="terms" default_title="Terms of Service" />
        },
        Route::About => html! {  // Add this block
            <MarkdownPage filename="about" default_title="About Us" />
        },
        Route::Post { id, title } => html! { <Post {id} {title} /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}
```

### 4. Add Navigation Links (Optional)

Update the footer or header to include a link to your new page:

```rust
<a href="/about">{ "About" }</a>
```

## How It Works

The `MarkdownPage` component:
1. Takes a `filename` prop (without the `.rmd` extension)
2. Loads the file from `posts/{filename}.rmd`
3. Parses the markdown content using `pulldown-cmark`
4. Renders it as HTML with the landing page styles

## Examples

### Privacy Page
- File: `posts/privacy.rmd` (symlink to `2022-08-01-Privacy-Notice.rmd`)
- Route: `/privacy`
- Usage: `<MarkdownPage filename="privacy" default_title="Privacy Policy" />`

### Terms Page
- File: `posts/terms.rmd` (symlink to `2022-07-15-TOS.rmd`)
- Route: `/terms`
- Usage: `<MarkdownPage filename="terms" default_title="Terms of Service" />`

## Supported Markdown Features

The markdown parser supports:
- Headings
- Links
- Lists (ordered and unordered)
- Bold and italic text
- Code blocks
- Tables
- Footnotes
- Strikethrough
- Task lists
- Smart punctuation

## Styling

Markdown pages automatically use the landing page design system with:
- Merriweather serif font for headings
- Inter sans-serif font for body text
- Responsive container layout
- Consistent spacing and typography

## Tips

1. **Use symlinks for existing files**: Instead of duplicating content, create symlinks:
   ```bash
   ln -s existing-file.rmd new-name.rmd
   ```

2. **Keep filenames simple**: Use lowercase and hyphens (e.g., `privacy-policy.rmd`)

3. **Test locally**: Build and run the project to test your new page:
   ```bash
   trunk serve
   ```

4. **Update navigation**: Don't forget to add links to your new page in the footer or header!
