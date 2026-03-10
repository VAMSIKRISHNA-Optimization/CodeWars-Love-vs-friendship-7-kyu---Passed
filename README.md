# CodeWars-Love-vs-friendship-7-kyu---Passed
Your task is to write a function which calculates the value of a word based off the sum of the alphabet positions of its characters.
If　a = 1, b = 2, c = 3 ... z = 26

Then l + o + v + e = 54

and f + r + i + e + n + d + s + h + i + p = 108

So friendship is twice as strong as love :-)

Your task is to write a function which calculates the value of a word based off the sum of the alphabet positions of its characters.

The input will always be made of only lowercase letters and will never be empty.


TEST CASES:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(words_to_marks("attitude"), 100);
        assert_eq!(words_to_marks("friends"), 75);
        assert_eq!(words_to_marks("family"), 66);
        assert_eq!(words_to_marks("selfness"), 99);
        assert_eq!(words_to_marks("knowledge"), 96);
    }
    
    const CHARSET: [char; 26] = [
      'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
      'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 
    ];

    use rand::{thread_rng, Rng};
    
    #[test]
    fn random() {
        let mut rng = thread_rng();
        
        for _ in 0..100 {
            let s = (0..rng.gen_range(0..CHARSET.len()))
                .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())])
                .collect::<String>();
            
            assert_eq!(words_to_marks(&s), words_to_marks_solution(&s));
        }
    }
    
    fn words_to_marks_solution(s: &str) -> u32 {
        s.bytes().map(|b| (b - 96) as u32).sum()
    }    
}
