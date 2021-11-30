template = '''#![allow(unused_variables)]

pub struct Solution;

impl crate::Solution for Solution {
  fn solve_1(&self, input: String) -> String {
      todo!()
  }

  fn solve_2(&self, input: String) -> String {
      todo!()
  }
}
'''

for i in range(1, 26):
    with open(f'src/day{i}.rs', 'w') as f:
        f.write(template)
