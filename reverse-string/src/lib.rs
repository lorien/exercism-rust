use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    //let mut ret = String::from("");
    //for graph in UnicodeSegmentation::graphemes(input, true)
    //    .rev()
    //    .collect::<Vec<&str>>()
    //{
    //    ret.push_str(graph);
    //}
    //ret
    UnicodeSegmentation::graphemes(input, true).rev().collect::<String>()
}
