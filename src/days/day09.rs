use crate::util::Answer;

pub fn solve(input: &str) -> anyhow::Result<String> {
    let mut dm: DiskMap = input.parse()?;

    let p1 = part_one(&mut dm);
    assert_eq!(p1, 6395800119709, "Part one is not correct.");

    let p2 = part_two(&mut dm);
    assert_eq!(p2, 6418529470362, "Part two is not correct.");

    Answer::first(9, p1).second(p2).report()
}

fn part_one(dm: &mut DiskMap) -> usize {
    dm.compact_blocks();
    dm.blocks_checksum()
}

fn part_two(dm: &mut DiskMap) -> usize {
    dm.compact_files();
    dm.files_checksum()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct File {
    start_pos: usize,
    file_id: usize,
    length: usize,
}

impl File {
    #[allow(dead_code)]
    fn new(start_pos: usize, file_id: usize, length: usize) -> Self {
        Self {
            start_pos,
            file_id,
            length,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Space {
    start_pos: usize,
    length: usize,
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
struct DiskMap {
    sparse_map: Vec<Block>,
    files: Vec<File>,
    spaces: Vec<Space>,
}

impl DiskMap {
    fn compact_blocks(&mut self) {
        let forward = 0..self.sparse_map.len();
        let mut backward = forward.clone().rev();

        for front_idx in forward {
            // Skip any files blocks.
            if self.sparse_map[front_idx].is_file() {
                continue;
            }
            // Now we have a space block, find the next file block at the end.
            let Some(back_idx) = backward.find(|idx| self.sparse_map[*idx].is_file()) else {
                return; // Exhausted the files.
            };
            if back_idx <= front_idx {
                // Our back cursor is before our front cursor, which means
                // there's no further compaction to perform.
                return;
            }
            // Swap the file block into the space block.
            self.sparse_map.swap(front_idx, back_idx);
        }
    }

    fn blocks_checksum(&self) -> usize {
        self.sparse_map
            .iter()
            .take_while(|b| b.is_file())
            .map(|b| match b {
                Block::File { id } => *id,
                _ => unreachable!(),
            })
            .enumerate()
            .map(|(idx, id)| idx * id)
            .sum()
    }

    fn compact_files(&mut self) {
        for file in self.files.iter_mut().rev() {
            // For each file, starting at the end,
            for space_index in 0..self.spaces.len() {
                // find an appropriate space, starting at the front, such that:
                let space = &mut self.spaces[space_index];
                // it is placed before the file,
                if space.start_pos > file.start_pos {
                    break;
                }
                // it can accommodate the length of the file,
                if space.length >= file.length {
                    file.start_pos = space.start_pos;
                    if space.length == file.length {
                        // perfectly,
                        self.spaces.remove(space_index);
                    } else {
                        // or with some remaining space.
                        space.start_pos += file.length;
                        space.length -= file.length;
                    }
                    break;
                }
            }
        }
    }

    fn files_checksum(&self) -> usize {
        self.files
            .iter()
            .flat_map(|f| {
                // Pair each occupied position with the occupying file's ID.
                let positions = f.start_pos..(f.start_pos + f.length);
                let repeated_id = std::iter::repeat(f.file_id);
                positions.zip(repeated_id)
            })
            .map(|(pos, file_id)| pos * file_id)
            .sum()
    }
}

impl std::str::FromStr for DiskMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ASCII_CODE_FOR_ZERO: u8 = 0x30;
        let all_lengths: Vec<usize> = s
            .trim()
            .bytes()
            .map(|b| (b - ASCII_CODE_FOR_ZERO) as usize)
            .collect();

        let sparse_size = all_lengths.iter().sum();
        let mut sparse_map: Vec<Block> = Vec::with_capacity(sparse_size);
        let mut files: Vec<File> = Vec::with_capacity(all_lengths.len());
        let mut spaces: Vec<Space> = Vec::with_capacity(all_lengths.len());

        let is_file_iter = [true, false].into_iter().cycle();
        let mut ids = 0..;
        let mut start_pos = 0_usize;

        for (is_file, length) in is_file_iter.zip(all_lengths.into_iter()) {
            if is_file {
                let file_id = ids.next().unwrap();
                files.push(File {
                    start_pos,
                    file_id,
                    length,
                });
                sparse_map.extend(std::iter::repeat_n(Block::File { id: file_id }, length));
            } else {
                spaces.push(Space { start_pos, length });
                sparse_map.extend(std::iter::repeat_n(Block::Free, length));
            }
            start_pos += length;
        }
        Ok(Self {
            sparse_map,
            files,
            spaces,
        })
    }
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two, Block, DiskMap, File, Space};

    const SAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parse_sample_input() -> anyhow::Result<()> {
        let map: DiskMap = SAMPLE_INPUT.parse()?;
        assert_eq!(
            map.files[0],
            File {
                start_pos: 0,
                file_id: 0,
                length: 2
            }
        );
        assert_eq!(
            map.spaces[0],
            Space {
                start_pos: 2,
                length: 3
            }
        );
        Ok(())
    }

    #[test]
    fn test_sparse_from_sample_input() -> anyhow::Result<()> {
        let map = SAMPLE_INPUT.parse::<DiskMap>()?;
        assert_eq!(map.sparse_map[0], Block::File { id: 0 });
        assert_eq!(map.sparse_map[1], Block::File { id: 0 });
        assert_eq!(map.sparse_map[2], Block::Free);
        Ok(())
    }

    #[test]
    fn test_compact_blocks() -> anyhow::Result<()> {
        let mut map = SAMPLE_INPUT.parse::<DiskMap>()?;
        map.compact_blocks();
        assert_eq!(map.sparse_map[0], Block::File { id: 0 });
        assert_eq!(map.sparse_map[1], Block::File { id: 0 });
        assert_eq!(map.sparse_map[2], Block::File { id: 9 });
        Ok(())
    }

    #[test]
    fn test_block_compact_checksum() -> anyhow::Result<()> {
        let mut dm = SAMPLE_INPUT.parse::<DiskMap>()?;
        let checksum = part_one(&mut dm);
        assert_eq!(checksum, 1928);
        Ok(())
    }

    #[test]
    fn test_sector_compact() -> anyhow::Result<()> {
        let mut map = SAMPLE_INPUT.parse::<DiskMap>()?;
        map.compact_files();
        assert!(map.files.contains(&File::new(0, 0, 2)));
        assert!(map.files.contains(&File::new(2, 9, 2)));
        assert!(map.files.contains(&File::new(4, 2, 1)));
        assert!(map.files.contains(&File::new(5, 1, 3)));
        assert!(map.files.contains(&File::new(8, 7, 3)));
        Ok(())
    }

    #[test]
    fn test_sector_compact_checksum() -> anyhow::Result<()> {
        let mut dm = SAMPLE_INPUT.parse::<DiskMap>()?;
        let checksum = part_two(&mut dm);
        assert_eq!(checksum, 2858);
        Ok(())
    }
}
