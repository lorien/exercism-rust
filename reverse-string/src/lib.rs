use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // 1st attemp
    //let mut ret = String::from("");
    //for graph in UnicodeSegmentation::graphemes(input, true)
    //    .rev()
    //    .collect::<Vec<&str>>()
    //{
    //    ret.push_str(graph);
    //}
    //ret
    //
    // 2nd attemp
    // UnicodeSegmentation::graphemes(input, true).rev().collect::<String>()

    input.graphemes(true).rev().collect::<String>()
}
