use tabled::{
    object::{Cell, Segment},
    Alignment, ModifyObject, Span, Style, TableIteratorExt,
};

fn main() {
    let data = [["just 1 column"; 5]; 5];

    let h_span = |r, c, span| Cell(r, c).modify().with(Span::column(span));
    let v_span = |r, c, span| Cell(r, c).modify().with(Span::row(span));

    let table = data
        .table()
        .with(h_span(0, 0, 5).with(|_: &str| "span all 5 columns".to_string()))
        .with(h_span(1, 0, 4).with(|_: &str| "span 4 columns".to_string()))
        .with(h_span(2, 0, 2).with(|_: &str| "span 2 columns".to_string()))
        .with(v_span(2, 4, 4).with(|_: &str| "just 1 column\nspan\n4\ncolumns".to_string()))
        .with(v_span(3, 1, 2).with(|_: &str| "span 2 columns\nspan\n2\ncolumns".to_string()))
        .with(h_span(3, 1, 2))
        .with(v_span(2, 3, 3).with(|_: &str| "just 1 column\nspan\n3\ncolumns".to_string()))
        .with(Style::modern())
        .with(Style::correct_spans())
        .with(Segment::all().modify().with(Alignment::center_vertical()));

    println!("{}", table);
}
