#[derive(Default)]
struct Parser {
    root: i32,          // vals[root] is the answer
    vals: Vec<f64>,     // Contains all numerical values

    /// For debugging
    ops: Vec<char>,     // Operator of each calculation * 'a' is numerical values
    left: Vec<i32>,     // Saved left-values
    right: Vec<i32>,    // Saved right-values
    ids: Vec<i32>,      // The values-index
    ind: i32,           // for ids
}

impl Parser {
    // Initialize
    #[no_mangle]
    pub extern "C" fn init(&mut self) {
        self.vals = Vec::new();
        self.ops = Vec::new();
        self.left = Vec::new();
        self.right = Vec::new();
        self.ids = Vec::new();
        self.ind = 0;
    }

    // push new value in vals
    fn new_node(&mut self, op: char, lp: i32, rp: i32, val: f64) -> i32 {
        self.ops.push(op);  self.left.push(lp);  self.right.push(rp);
        if op == 'a' {                      // if a number
            self.vals.push(val);
            self.ids.push(self.ind);        // input numerical value
            self.ind += 1;
        } else {                            // if a not number
            if      op == '+' {self.vals.push(self.vals[lp as usize] + self.vals[rp as usize]);}
            else if op == '-' {self.vals.push(self.vals[lp as usize] - self.vals[rp as usize]);}
            else if op == '*' {self.vals.push(self.vals[lp as usize] * self.vals[rp as usize]);}
            else if op == '/' {self.vals.push(self.vals[lp as usize] / self.vals[rp as usize]);}
            self.ids.push(-1);              // calculation result index
        }
        return self.vals.len() as i32 - 1;
    }

    // main function
    #[no_mangle]
    pub extern "C" fn solve(&mut self, s: &str) -> f64 {
        let mut _p: usize = 0;                  // n_s's pointer
        let mut n_s: String = String::from("");
        // skip blank
        for c in s.chars() {
            if c != ' ' {
                n_s.push(c);
            }
        }
        self.root = self.expr(&n_s, &mut _p);
        return self.vals[self.root as usize];   // answer
    }

    // priority 3th -- '+' or '-'
    fn expr(&mut self, s: &str, p: &mut usize) -> i32 {
        let mut lp: i32 = self.factor(s, p);
        while *p < s.len() as usize && (&s[*p..(*p + 1)] == "+" || &s[*p..(*p + 1)] == "-") {
            let mut op: char = '-';
            if &s[*p..(*p + 1)] == "+" {op = '+';}
            *p += 1;
            let rp: i32 = self.factor(s, p);
            lp = self.new_node(op, lp, rp, 0.0)
        }
        return lp;
    }

    // priority 2nd -- '*' or '/'
    fn factor(&mut self, s: &str, p: &mut usize) -> i32 {
        let mut lp: i32 = self.value(s, p);
        while *p < s.len() as usize && (&s[*p..(*p + 1)] == "*" || &s[*p..(*p + 1)] == "/") {
            let mut op: char = '/';
            if &s[*p..(*p + 1)] == "*" {op = '*';}
            *p += 1;
            let rp: i32 = self.value(s, p);
            lp = self.new_node(op, lp, rp, 0.0)
        }
        return lp;
    }

    // priority 1st -- '(' or ')'
    fn value(&mut self, s: &str, p: &mut usize) -> i32 {
        if &s[*p..(*p + 1)] == "(" {
            *p += 1;
            let lp: i32 = self.expr(s, p);
            *p += 1;
            return lp;
        } else {
            let mut val: f64 = 0.0; let mut decimal = 0;
            while (*p < s.len() as usize) && ((&s[*p..(*p + 1)] >= "0" && &s[*p..(*p + 1)] <= "9") || &s[*p..(*p + 1)] == ".") {
                if  &s[*p..(*p + 1)] == "." {
                    if decimal > 0 {
                        *p += 1;
                        continue;
                    }
                    decimal = 1;
                    *p += 1;
                    continue;
                }
                let str_num = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
                for num in (0..10) {
                    if &s[*p..(*p + 1)] == str_num[num] {
                        if decimal == 0 {
                            val = val * 10.0 + num as f64;
                        } else {
                            val = val + num as f64 / 10_i32.pow(decimal) as f64;
                            decimal += 1;
                        }
                        break;
                    }
                }
                *p += 1;
            }
            return self.new_node('a', -1, -1, val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve() {
        let mut final_ans: Parser = Default::default();
        final_ans.init();
        assert_eq!(25.0, final_ans.solve("12+13"));
    }
}
