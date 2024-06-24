Little routine to replace the annoying /Fit destination on Page Objects
so bookmark links don't resize the page everytime one clicks on one.

Replaces '[ page /Fit ]' with '[ page /XYZ none none none ]'
(See: [StackExchange answer](https://superuser.com/questions/278302/prevent-adobe-reader-from-switching-to-fit-page-zoom-when-bookmark-is-clicked/1770617#1770617))

Usage
`
pdf_tune [valid_path_to_pdf]
`
Creates a new pdf file with '\[filename](_new).pdf' at the same location

`
pdf_tune [valid_path_to_pdf] replace
`
Replaces original pdf file with the updated content (Use at your own risk).

---

If it doesn't work, it may be that the destination defintions for the bookmarks are compressed and not 'readable'.  
(1),(2)

Only solution I have found so far is to uncompress the pdf file (with [pdftk](http://www.pdflabs.com/docs/pdftk-man-page/#dest-compress) or [QPDF](http://qpdf.sourceforge.net/))

re-run the script with the uncompressed pdf

and compress it again (if you care about file size).. 

--

(1) - https://unix.stackexchange.com/questions/269033/how-to-remove-zoom-information-from-a-pdf-bookmark  
(2) - https://unix.stackexchange.com/questions/17663/how-to-know-if-a-pdf-file-is-compressed-or-not-and-to-uncompress-it


