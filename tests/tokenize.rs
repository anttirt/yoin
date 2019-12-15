extern crate yoin;

#[test]
fn tokenize_works() {
    let input = "すもももももももものうち";
    let tokenizer = yoin::ipadic::tokenizer();
    let expected_tokens =
        vec![("すもも",
              vec!["名詞", "一般", "*", "*", "*", "*", "すもも", "スモモ", "スモモ"]),
             ("も", vec!["助詞", "係助詞", "*", "*", "*", "*", "も", "モ", "モ"]),
             ("もも",
              vec!["名詞", "一般", "*", "*", "*", "*", "もも", "モモ", "モモ"]),
             ("も", vec!["助詞", "係助詞", "*", "*", "*", "*", "も", "モ", "モ"]),
             ("もも",
              vec!["名詞", "一般", "*", "*", "*", "*", "もも", "モモ", "モモ"]),
             ("の", vec!["助詞", "連体化", "*", "*", "*", "*", "の", "ノ", "ノ"]),
             ("うち",
              vec!["名詞",
                   "非自立",
                   "副詞可能",
                   "*",
                   "*",
                   "*",
                   "うち",
                   "ウチ",
                   "ウチ"])];
    let tokens = tokenizer.tokenize(input);
    assert_eq!(tokens.len(), expected_tokens.len());
    for (t, (expected_surface, expected_features)) in tokens.into_iter().zip(expected_tokens) {
        assert_eq!(t.surface(), expected_surface);
        assert_eq!(t.features().collect::<Vec<_>>(), expected_features);
    }
}

#[test]
fn tokenize_testcase_1() {
    let input = "魔神の御稚児。 ";
    let tokenizer = yoin::ipadic::tokenizer();
    for token in tokenizer.tokenize(input) {
        print!("{} ", token.surface());
        for feat in token.features() {
            print!("{},", feat);
        }
        println!();
    }
    println!("done");
}

#[test]
fn tokenize_testcase_2() {
    let input = "菜穂子";
    let tokenizer = yoin::ipadic::tokenizer();
    for token in tokenizer.tokenize(input) {
        print!("{} ", token.surface());
        for feat in token.features() {
            print!("{},", feat);
        }
        println!();
    }
    println!("done");
}

#[test]
fn tokenize_testcase_3() {
    let input = "ユナ";
    let tokenizer = yoin::ipadic::tokenizer();
    for token in tokenizer.tokenize(input) {
        print!("{} ", token.surface());
        for feat in token.features() {
            print!("{},", feat);
        }
        println!();
    }
    println!("done");
}