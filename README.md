# css-inline

[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/Stranger6667/css-inline/build.yml?style=flat-square&labelColor=555555&logo=github" height="20">](https://github.com/Stranger6667/css-inline)
[<img alt="crates.io" src="https://img.shields.io/crates/v/css-inline.svg?style=flat-square&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/css-inline)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-css_inline-66c2a5?style=flat-square&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/css-inline)
[<img alt="codecov.io" src="https://img.shields.io/codecov/c/gh/Stranger6667/css-inline?logo=codecov&style=flat-square&token=tOzvV4kDY0" height="20">](https://app.codecov.io/github/Stranger6667/css-inline)
[<img alt="gitter" src="https://img.shields.io/gitter/room/Stranger6667/css-inline?style=flat-square" height="20">](https://gitter.im/Stranger6667/css-inline)

`css-inline` is a crate that inlines CSS into HTML documents, built using components from Mozilla's Servo project.

This process is essential for sending HTML emails as you need to use "style" attributes instead of "style" tags.

For instance, the crate transforms HTML like this:

```html
<html>
    <head>
        <title>Test</title>
        <style>h1 { color:blue; }</style>
    </head>
    <body>
        <h1>Big Text</h1>
    </body>
</html>
```

into:

```html
<html>
    <head>
        <title>Test</title>
    </head>
    <body>
        <h1 style="color:blue;">Big Text</h1>
    </body>
</html>
```

## Installation

To include it in your project, add the following line to the dependencies section in your project's `Cargo.toml` file:

```toml
[dependencies]
css-inline = "0.9"
```

The Minimum Supported Rust Version is 1.60.

## Usage

```rust
const HTML: &str = r#"<html>
<head>
    <title>Test</title>
    <style>h1 { color:blue; }</style>
</head>
<body>
    <h1>Big Text</h1>
</body>
</html>"#;

fn main() -> Result<(), css_inline::InlineError> {
    let inlined = css_inline::inline(HTML)?;
    // Do something with inlined HTML, e.g. send an email
    Ok(())
}
```

### Features & Configuration

`css-inline` can be configured by using `CSSInliner::options()` that implements the Builder pattern:

```rust
const HTML: &str = "...";

fn main() -> Result<(), css_inline::InlineError> {
    let inliner = css_inline::CSSInliner::options()
        .load_remote_stylesheets(false)
        .build();
    let inlined = inliner.inline(HTML);
    // Do something with inlined HTML, e.g. send an email
    Ok(())
}
```

- `keep_style_tags`. Specifies whether to keep "style" tags after inlining. Default: `false`
- `base_url`. The base URL used to resolve relative URLs. If you'd like to load stylesheets from your filesystem, use the `file://` scheme. Default: `None`
- `load_remote_stylesheets`. Specifies whether remote stylesheets should be loaded. Default: `true`
- `extra_css`. Extra CSS to be inlined. Default: `None`
- `preallocate_node_capacity`. **Advanced**. Preallocates capacity for HTML nodes during parsing. This can improve performance when you have an estimate of the number of nodes in your HTML document. Default: `8`

You can also skip CSS inlining for an HTML tag by adding the `data-css-inline="ignore"` attribute to it:

```html
<head>
    <title>Test</title>
    <style>h1 { color:blue; }</style>
</head>
<body>
    <!-- The tag below won't receive additional styles -->
    <h1 data-css-inline="ignore">Big Text</h1>
</body>
```

The `data-css-inline="ignore"` attribute also allows you to skip `link` and `style` tags:

```html
<head>
    <title>Test</title>
    <!-- Styles below are ignored -->
    <style data-css-inline="ignore">h1 { color:blue; }</style>
</head>
<body>
    <h1>Big Text</h1>
</body>
```

If you'd like to load stylesheets from your filesystem, use the `file://` scheme:

```rust
const HTML: &str = "...";

fn main() -> Result<(), css_inline::InlineError> {
    let base_url = css_inline::Url::parse("file://styles/email/").expect("Invalid URL");
    let inliner = css_inline::CSSInliner::options()
        .base_url(Some(base_url))
        .build();
    let inlined = inliner.inline(HTML);
    // Do something with inlined HTML, e.g. send an email
    Ok(())
}
```

## Standards support & restrictions

`css-inline` is built on top of [html5ever](https://crates.io/crates/html5ever) and [cssparser](https://crates.io/crates/cssparser) and relies on their behavior for HTML & CSS parsing.

- Only HTML 5 is supported, not XHTML.
- Only CSS 3 is supported.
- Only UTF-8 encoding for string representation. Other document encodings are not yet supported.

## Bindings

We provide bindings for Python and WebAssembly. Check the `bindings` directory for more information.

## Command Line Interface

`css-inline` provides a command-line interface:

```text
css-inline --help

css-inline inlines CSS into HTML documents.

USAGE:
   css-inline [OPTIONS] [PATH ...]
   command | css-inline [OPTIONS]

ARGS:
    <PATH>...
        An HTML document to process. In each specified document
        "css-inline" will look for all relevant "style" and "link"
        tags, will load CSS from them and then inline it to the
        HTML tags, according to the corresponding CSS selectors.
        When multiple documents are specified, they will be
        processed in parallel, and each inlined file will be saved
        with an "inlined." prefix. For example, for "example.html",
        there will be "inlined.example.html".

OPTIONS:
    --inline-style-tags
        Specifies whether to inline CSS from "style" tags.
        To disable inlining from "style" tags use
        `--inline-style-tags=false`.

    --keep-style-tags
        Keep "style" tags after inlining.

    --base-url
        The base URL used to resolve relative URLs.

    --load-remote-stylesheets
        Specifies if remote stylesheets should be loaded or not.

    --extra-css
        Additional CSS to inline.

    --output-filename-prefix
        Custom prefix for output files. Defaults to `inlined.`.
```

## Extra materials

If you're interested in learning how this library was created and how it works internally, check out these articles:

- [Rust crate](https://dygalo.dev/blog/rust-for-a-pythonista-2/)
- [Python bindings](https://dygalo.dev/blog/rust-for-a-pythonista-3/)

## Support

If you have any questions or discussions related to this library, please join our [gitter](https://gitter.im/Stranger6667/css-inline)!

## License

This project is licensed under the terms of the <a href="LICENSE">MIT license</a>.
