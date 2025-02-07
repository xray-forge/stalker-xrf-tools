use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_utils::assert_length;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotionMark {
  pub name: String,
  pub intervals: Vec<(f32, f32)>,
}

impl ChunkReadWrite for OgfMotionMark {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let name: String = reader.read_w1251_rn_string()?;

    let count: u32 = reader.read_u32::<T>()?;
    let mut intervals: Vec<(f32, f32)> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      intervals.push((reader.read_f32::<T>()?, reader.read_f32::<T>()?));
    }

    assert_length(
      &intervals,
      count as usize,
      "Expected correct count of OGF mark intervals to be read",
    )?;

    Ok(Self { name, intervals })
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
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
