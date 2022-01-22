#[derive(Debug, PartialEq)]
pub struct Solution {
  vec_2d: Vec<Vec<isize>>,
  row: usize,
  col: usize,
}

/* key takeaways
   - we might oversimplify this by ignoring
     the cases where some of the vec(s) might
     be empty:
     [[],[],[4]]
   - this also means the indexes you recorded
     in the row and col might not be pointing
     at a valid location
   - in the above case, the initial indexes
     [0][0] is really not pointing at a
     valid location
     - you need to skips the first two rows
       before you can reach at a valid location

*/

impl Solution {
  pub fn from(vec_2d: Vec<Vec<isize>>) -> Solution {
    Solution {
      vec_2d: vec_2d,
      row: 0,
      col: 0,
    }
  }

  pub fn next(&mut self) -> Option<isize> {
    if !(self.row < self.vec_2d.len()) {
      return None;
    }
    /*
      - you might be wondering why we are even doing this;
        it seems overcomplicated and unncessary.
      - well, vec_2d[0][0], does garantee to have a valid
        value
        - let say if vec_2d = [[],[],[2]]
        - next should be able to return 2, which is actually
          vec_2d[2][0]
    */
    if self.row < self.vec_2d.len() && self.col >= self.vec_2d[self.row].len() {
      /* skips those rows that don't have any elements */
      while self.row < self.vec_2d.len() {
        self.row += 1;
        if self.col < self.vec_2d[self.row].len() {
          break;
        }
      }
    }

    let val = self.vec_2d[self.row][self.col];

    /* at the end of an array */
    if self.col == self.vec_2d[self.row].len() - 1 {
      self.col = 0;
      self.row += 1;
    } else {
      self.col += 1;
    }
    Some(val)
  }

  pub fn has_next(&self) -> bool {
    /* ok vec_2d[row][col] is a valid location */
    if self.row < self.vec_2d.len() && self.col < self.vec_2d[self.row].len() {
      return true;
    } else if self.row < self.vec_2d.len() {
      let mut tmp_row = self.row;

      /* skip empty rows if needed */
      while tmp_row < self.vec_2d.len() {
        if self.col < self.vec_2d[tmp_row].len() {
          return true;
        }
        tmp_row += 1;
      }
    }

    /*
      vec_2d[row][col] is not a valid location
    */
    false
  }

  pub fn test_fixrure_1() -> Vec<Vec<isize>> {
    vec![vec![1, 2], vec![3], vec![4]]
  }
  pub fn test_fixrure_2() -> Vec<Vec<isize>> {
    vec![vec![], vec![], vec![4]]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let mut sn = Solution::from(Solution::test_fixrure_1());

    assert_eq!(sn.next(), Some(1));
    assert_eq!(sn.next(), Some(2));
    assert_eq!(sn.next(), Some(3));
    assert_eq!(sn.has_next(), true);
    assert_eq!(sn.has_next(), true);
    assert_eq!(sn.next(), Some(4));
    assert_eq!(sn.has_next(), false);
  }
}
