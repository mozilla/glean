use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

use pulldown_cmark::Event::{End, Start, Text};
use pulldown_cmark::Tag::List;
use pulldown_cmark::TagEnd;
use pulldown_cmark_to_cmark::cmark;
use xshell::{cmd, Shell};

#[test]
fn reference_metrics_toc_is_sorted() {
    // Relative to `glean-core`
    let index_file = "../docs/user/reference/metrics/index.md";
    let src = fs::read_to_string(index_file).expect("unable to read file");
    let parser = pulldown_cmark::Parser::new(&src);
    let mut events = parser.into_offset_iter();

    let mut modified_events = Vec::new();

    let mut output = File::create(index_file).unwrap();

    // Everything before the list
    let mut list_start = 0;
    for (event, span) in events.by_ref() {
        if let Start(List(..)) = event {
            modified_events.push(event);
            list_start = span.start;
            break;
        }
    }

    // Output everything up to the list start unmodified.
    if list_start > 0 {
        writeln!(&mut output, "{}", &src[0..(list_start - 1)]).unwrap();
    }

    let mut items = HashMap::new();

    // The list itself.
    // We store a `title -> events` mapping to sort it later by `title`.
    let mut list_end = 0;
    while let Some((event, span)) = events.next() {
        if let End(TagEnd::List(..)) = event {
            list_end = span.end;
            break;
        }

        let mut elems = Vec::new();
        elems.push(event);

        let mut title: Option<String> = None;
        for (event, _) in events.by_ref() {
            match event {
                End(TagEnd::Item) => {
                    elems.push(End(TagEnd::Item));
                    break;
                }
                Text(s) if title.is_none() => {
                    title = Some(s.to_string());
                    elems.push(Text(s));
                }
                e => elems.push(e),
            }
        }
        items.insert(title.unwrap(), elems);
    }

    let mut item_keys: Vec<_> = items.keys().cloned().collect();
    item_keys.sort_by_key(|item| {
        // We need to handle a few special cases:
        //
        // * Dual-labeled and labeled metrics should be sorted right after their top-level metric type
        // * Normalizing plurals to be sorted correctly after the singular (quantity -> quantities)

        let mut k = item.to_ascii_lowercase();
        k = k.strip_prefix("dual-labeled ").unwrap_or(&k).to_string();
        k = k.strip_prefix("labeled ").unwrap_or(&k).to_string();

        match &*k {
            _ if item == "Dual-labeled counters" => String::from("counter3"),
            "counters" => String::from("counter2"),
            "quantities" => String::from("quantity2"),
            "strings" => String::from("string2"),
            "string list" => String::from("string3"),
            _ => k,
        }
    });

    for key in item_keys {
        let elems = items.remove(&key).unwrap();
        modified_events.extend(elems);
    }
    modified_events.push(End(TagEnd::List(false)));

    let mut output_markdown = String::new();
    cmark(modified_events.into_iter(), &mut output_markdown).unwrap();
    writeln!(&mut output, "{output_markdown}\n").unwrap();

    // The remaining document can be printed unmodified.
    if list_end > 0 {
        write!(&mut output, "{}", &src[list_end..]).unwrap();
    }

    // Last but not least check if we modified the document.
    let sh = Shell::new().unwrap();
    cmd!(sh, "git --no-pager diff --exit-code {index_file}")
        .run()
        .unwrap();
}
