Little routine to replace the annoying /Fit destination on Page Objects
so Bookmark links don't resize the page everytime one clicks on one.

Replaces '[ page /Fit ]' with '[ page /XYZ none none none ]'
(See: [StackExchange answer](https://superuser.com/questions/278302/prevent-adobe-reader-from-switching-to-fit-page-zoom-when-bookmark-is-clicked/1770617#1770617))

Usage
`
pdf_tune [valid_path_to_pdf]
`
Creates a new pdf file with [filename](new).pdf

`
pdf_tune [valid_path_to_pdf] replace
`
Replaces original pdf file with the updated content (Use at your own risk). 