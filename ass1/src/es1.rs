use unicode_segmentation::UnicodeSegmentation;

pub fn string_reverse(str: &str) -> String {
    str.graphemes(true).rev().collect()
}

pub fn es1() {
    println!("Ciao io sono {}", string_reverse("ociredeF"));

    println!(
        "Il reverse di questa frase è->{}",
        string_reverse("Il reverse di questa frase è ")
    );

    println!("Anche नमस्ते->{}", string_reverse("Anche नमस्ते->"));
}
