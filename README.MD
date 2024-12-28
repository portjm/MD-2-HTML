# MD 2 HTML
(work in progress) 

Simple utility to convert your Markdown files into formatted HTML, written in Rust. 

##### Usage
```
./M2H <filepath>
```

## Dev Log
###### 12/28/24 - Parsing Markdown

###### 12/27/24 - Basic CONOPS
##### Goal
To provide an easy way to generate static HTML pages automatically from Markdown documents with minimal set-up and dependencies.

##### Background/Motivations
I recently tried to set up a website using GitHub Pages (GHP) and Jekyll. I'd never used Jekyll before (or Ruby for that matter) but it seemed like it'd be easy to set up (I have 4/5 of a BS in computer science after all); 10 min into trying to install it, I was still wrestling with Ruby's package manager so I gave up. 

I realize this is a minor issue that could have been solved with a bit of patience but my point is that I went with Jekyll because I thought it'd be easy to use; Instead, I found myself reading documentation for the package manager of a programming language that I'll likely never use.

All I wanted was to generate static pages from Markdown.

##### Primary Use Cases
*   **Web Publishing:** A content creator writes articles in Markdown and uses the utility to generate static HTML files. These files can then be deployed to a web server for direct access by users. 

*   **Static Content Generation within Web Frameworks:** The utility is used during the build process of the web application. Markdown files are converted to HTML as part of the build process, resulting in static HTML pages or components being generated.

*   **Documentation Generation:** A software developer uses the utility to convert Markdown documentation files into HTML for inclusion in project documentation or online help systems.


