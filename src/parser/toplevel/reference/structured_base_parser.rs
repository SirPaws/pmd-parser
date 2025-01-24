pub fn peek_next_token(content: &str) -> Option<String> {
    let mut text : String = "".into();
    let mut peekable_content = content.trim_start().chars().peekable();
    while let Some(character) = peekable_content.peek(){
        if character.is_whitespace() && text.len() == 0{
            peekable_content.next();
            continue;
        } else if character.is_whitespace() {
            break;
        }

        match character {
            &':' | &'{' | &'}' | &',' | &'[' | &']' => {
                if text.len() > 0 {
        
                    let mut lookahead = peekable_content.clone();
                    lookahead.next();
                    if let Some(slash) = lookahead.peek() && slash == &'/' {
                        text.push(':');
                    } else {
                        break;
                    }
                } else {
                    return Some(character.to_string());
                }
            },
            &'"' => {
                peekable_content.next();
                if text.len() > 0 {
                    break;
                } else {
                    text.push('"');
                    while let Some(chr) = peekable_content.peek() && chr != &'"' {
                        if chr == &'\n' { break }
                        text.push(*chr);
                        peekable_content.next();
                    }
                    if let Some(chr) = peekable_content.peek() && chr == &'"' {
                        peekable_content.next();
                    }

                    text.push('"');
                    break;
                }
            }
            _ => {
                text.push(*character)
            }
        }
        peekable_content.next();
    }

    if text.len() == 0 { None } else { Some(text) }
}

pub fn eat_token(content: &str, token: &str) -> String {
    content.trim_start()[token.len()..].to_string()
}

pub fn parse_value(text: &str) -> (String, Vec<String>) {
    let opt_first_token = peek_next_token(text);

    if opt_first_token.is_none() || opt_first_token.ne(&Some("[".to_string())) {
        let token = opt_first_token.unwrap();
        if token.starts_with('"') && token.ends_with('"') {
            let buf = eat_token(text, token.as_str());
            let text = &token[1..];
            (buf, vec![text[..text.len() - 1].to_string()])
        } else {
            let mut buf = text.to_string();
            while buf.len() != 0 && buf.chars().nth(0).is_some_and(|x| !(x == ',' || x == '\n')) {
                buf = buf.chars().skip(1).collect();
            };
            
            let diff = text.len() - buf.len();
            let result = (buf, vec![text[0..diff].trim().to_string()]);
            result
        }
    } else {
        let mut buf = eat_token(text, "[");
        let mut names = vec![];
        while let Some(tk) = peek_next_token(&buf) {
            if tk == "]" { break; }

            let mut name = tk.to_string();

            buf = eat_token(&buf, &tk);
            while let Some(comma) = peek_next_token(&buf) && comma != "," {
                if comma == "]" { break; }
                name += " "; 
                name += comma.as_str();
                buf = eat_token(&buf, &comma);
            }
            if let Some(comma) = peek_next_token(&buf) && comma == "," {
                buf = eat_token(&buf, &comma);
            }

            if name.trim().starts_with('"') && name.trim().ends_with('"') {
                name = name[1..].to_string();
                name = name[0..name.len() - 1].to_string();
            }
            names.push(name.trim().to_string());
        }

        if let Some(tk) = peek_next_token(&buf) && tk == "]" {
            buf = eat_token(&buf, "]");
        }

        (buf, names)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  peek_simple_token() {
        let example_text = "token";
        let example = peek_next_token(example_text);
    
        assert!(example.is_some());
        assert_eq!(example.unwrap(), "token");
    }
    
    #[test]
    fn  peek_simple_token_with_extra_tokens_after() {
        let example_text = "token something else";
        let example = peek_next_token(example_text);
    
        assert!(example.is_some());
        assert_eq!(example.unwrap(), "token");
    }
   

    #[test]
    fn  peek_simple_token_with_whitespace_in_front() {
        let example_text = "   token";
        let example = peek_next_token(example_text);
    
        assert!(example.is_some());
        assert_eq!(example.unwrap(), "token");
    }
    
    #[test]
    fn  eat_simple_token() {
        let example_text = "token something else";
        let example = peek_next_token(example_text);
    
        assert!(example.is_some());
        let example = example.unwrap();
        assert_eq!(example, "token");
        
        let remainder = eat_token(example_text, example.as_str());
        assert_eq!(remainder, " something else");
    }
    
    #[test]
    fn  eat_simple_token_with_whitespace() {
        let example_text = "     token something else";
        let example = peek_next_token(example_text);
    
        assert!(example.is_some());
        let example = example.unwrap();
        assert_eq!(example, "token");
        
        let remainder = eat_token(example_text, example.as_str());
        assert_eq!(remainder, " something else");
    }


    #[test]
    fn value_parsing_simple() {
        let example_text = "this is just a string";
        
        let (buf, example) = parse_value(example_text);

        assert_eq!(example, vec!["this is just a string"]);
        assert_eq!(buf, "");
    }
    
    #[test]
    fn value_parsing_string() {
        let example_text = "\"this is just, a string\"";
        
        let (buf, example) = parse_value(example_text);

        assert_eq!(example, vec!["this is just, a string"]);
        assert_eq!(buf, "");
    }
    
    #[test]
    fn value_parsing_url() {
        let example_text = "protocol://hereis.asite.com/";
        
        let (buf, example) = parse_value(example_text);

        assert_eq!(example, vec!["protocol://hereis.asite.com/"]);
        assert_eq!(buf, "");
    }
    
    #[test]
    fn value_parsing_array_empty() {
        let example_text = "[]";
        
        let (buf, example) = parse_value(example_text);

        assert_eq!(example, Vec::<String>::new());
        assert_eq!(buf, "");
    }
    
    #[test]
    fn value_parsing_array_one_value() {
        let example_text = "[this is just a string]";
        
        let (buf, example) = parse_value(example_text);

        assert_eq!(example, vec!["this is just a string"]);
        assert_eq!(buf, "");
    }
    
    #[test]
    fn value_parsing_array_one_string() {
        let example_text = "[\"this is just, a string\"]";
        
        let (buf, example) = parse_value(example_text);

        assert_eq!(example, vec!["this is just, a string"]);
        assert_eq!(buf, "");
    }
    
    #[test]
    fn value_parsing_array_multiple_values() {
        let example_text = "[this is just a string, this is a different string]";
        
        let (buf, example) = parse_value(example_text);

        assert_eq!(example, vec!["this is just a string", "this is a different string"]);
        assert_eq!(buf, "");
    }
    
    #[test]
    fn value_parsing_array_multiple_strings() {
        let example_text = "[\"this is just, a string\", \"this is, a different string\"]";
        
        let (buf, example) = parse_value(example_text);

        assert_eq!(example, vec!["this is just, a string", "this is, a different string"]);
        assert_eq!(buf, "");
    }

}
