pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let dy = [-1, -1, -1,  0, 0,  1, 1, 1];
    let dx = [-1,  0,  1, -1, 1, -1, 0, 1];


    let row = minefield.len() as i32;
    let col = minefield.get(0).unwrap_or(&"").len() as i32;
    let mut ret = vec![];

    for i in 0..row {
        let mut line = String::new();
        for j in 0..col {

            let mut cur = minefield[i as usize].chars().nth(j as usize).unwrap();
            
            // handle white space
            if cur.is_whitespace() {
                let mut around_mine_cnt = 0;

                // check around            
                for d in 0..8 {
                    let ny = dy[d] + i;
                    let nx = dx[d] + j;
                    if ny < 0 || nx < 0 || ny >= row || nx >= col {
                        continue;
                    }
                    let around = minefield[ny as usize].chars().nth(nx as usize).unwrap();
                    if around == '*' {
                        around_mine_cnt += 1;
                    }
                }

                if around_mine_cnt != 0 {
                    cur = char::from_digit(around_mine_cnt, 10).unwrap();
                }
            }
            line.push(cur);
        }
        ret.push(line);
    }
    ret
}
