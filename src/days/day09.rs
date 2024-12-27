use std::{ops::Index, str::FromStr};

use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let dm: DenseMap = input.parse()?;

    let p1 = part_one(&dm);
    assert_eq!(p1, 6395800119709, "Part one is not correct.");

    Answer::first(9, p1).report()
}

fn part_one(dm: &DenseMap) -> usize {
    let mut sm = dm.expand();
    sm.compact_blocks();
    sm.checksum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Sector {
    File { id: usize, length: u8 },
    Free { length: u8 },
}

#[derive(Debug, Clone)]
struct DenseMap(Vec<Sector>);

impl DenseMap {
    fn expand(&self) -> SparseMap {
        let expanded_len = self
            .0
            .iter()
            .map(|s| match s {
                Sector::File { id: _, length } => *length as usize,
                Sector::Free { length } => *length as usize,
            })
            .sum::<usize>();
        let mut sparse = Vec::with_capacity(expanded_len);
        for sector in &self.0 {
            let it = match sector {
                Sector::File { id, length } => {
                    std::iter::repeat_n(Block::File { id: *id }, *length as usize)
                }
                Sector::Free { length } => std::iter::repeat_n(Block::Free, *length as usize),
            };
            sparse.extend(it);
        }

        SparseMap(sparse)
    }
}

impl FromStr for DenseMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let all: Vec<u8> = s.trim().bytes().map(|b| b - 48).collect();
        let mut res = Vec::with_capacity(all.len());
        let mut ids = 0..;

        for chunk in all.chunks(2) {
            if let [file_length, free_length] = chunk {
                let file = Sector::File {
                    id: ids.next().unwrap(),
                    length: *file_length,
                };
                let free = Sector::Free {
                    length: *free_length,
                };
                res.push(file);
                res.push(free);
            } else if let [file_length] = chunk {
                let file = Sector::File {
                    id: ids.next().unwrap(),
                    length: *file_length,
                };
                res.push(file);
            }
        }

        Ok(Self(res))
    }
}

impl Index<usize> for DenseMap {
    type Output = Sector;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
    File { id: usize },
    Free,
}

impl Block {
    /// Returns `true` if the block is [`File`].
    ///
    /// [`File`]: Block::File
    #[must_use]
    fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }
}

#[derive(Debug, Clone)]
struct SparseMap(Vec<Block>);

impl SparseMap {
    fn compact_blocks(&mut self) {
        let forward = 0..self.0.len();
        let mut backward = forward.clone().rev();

        for front_idx in forward {
            if self.0[front_idx].is_file() {
                continue;
            }
            // Now we're at free space.
            // So find the next file block at the end.
            let Some(back_idx) = backward.find(|idx| self.0[*idx].is_file()) else {
                return; // Exhausted the files.
            };
            if back_idx <= front_idx {
                // We're finished.
                return;
            }
            self.0.swap(front_idx, back_idx);
        }
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .take_while(|b| b.is_file())
            .map(|b| match b {
                Block::File { id } => *id,
                _ => 0,
            })
            .enumerate()
            .map(|(idx, id)| idx * id)
            .sum()
    }
}

impl Index<usize> for SparseMap {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod test {
    use super::{part_one, Block, DenseMap, Sector};

    const SAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parse_sample_input() -> anyhow::Result<()> {
        let map: DenseMap = SAMPLE_INPUT.parse()?;
        assert_eq!(map[0], Sector::File { id: 0, length: 2 });
        assert_eq!(map[1], Sector::Free { length: 3 });
        Ok(())
    }

    #[test]
    fn test_sparse_from_sample_input() -> anyhow::Result<()> {
        let map = SAMPLE_INPUT.parse::<DenseMap>()?.expand();
        assert_eq!(map[0], Block::File { id: 0 });
        assert_eq!(map[1], Block::File { id: 0 });
        assert_eq!(map[2], Block::Free);
        Ok(())
    }

    #[test]
    fn test_compact_blocks() -> anyhow::Result<()> {
        let mut map = SAMPLE_INPUT.parse::<DenseMap>()?.expand();
        map.compact_blocks();
        assert_eq!(map[0], Block::File { id: 0 });
        assert_eq!(map[1], Block::File { id: 0 });
        assert_eq!(map[2], Block::File { id: 9 });
        Ok(())
    }

    #[test]
    fn test_compact() -> anyhow::Result<()> {
        let dm = SAMPLE_INPUT.parse::<DenseMap>()?;
        let checksum = part_one(&dm);
        assert_eq!(checksum, 1928);
        Ok(())
    }
}
