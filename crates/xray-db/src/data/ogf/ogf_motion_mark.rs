use crate::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{assert_chunk_vector_read, ChunkReader, ChunkWriter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotionMark {
  pub name: String,
  pub intervals: Vec<(f32, f32)>,
}

impl OgfMotionMark {
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let name: String = reader.read_rn_terminated_win_string()?;

    let count: u32 = reader.read_u32::<T>()?;
    let mut intervals: Vec<(f32, f32)> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      intervals.push((reader.read_f32::<T>()?, reader.read_f32::<T>()?));
    }

    assert_chunk_vector_read(
      &intervals,
      count as usize,
      "Expected correct count of OGF mark intervals to be read",
    )?;

    Ok(Self { name, intervals })
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}

/*
sub read_motion_mark {
  my $self = $_[0];
  $self->{name} = '';
  my $c;
  while (1) {
    ($c) = $_[1]->unpack('a');
    last if ($c eq "\n" || $c eq "\r");
    $self->{name} .= $c;
  }
  ($c) = $_[1]->unpack('a');
  die unless $c eq "\n";
  my ($count) = $_[1]->unpack('V', 4);
  for (my $i = 0; $i < $count; $i++) {
    my $int = {};
    ($int->{min}, $int->{max}) = $_[1]->unpack('ff', 8);
    push @{$self->{intervals}}, $int;
  }
}
 */
