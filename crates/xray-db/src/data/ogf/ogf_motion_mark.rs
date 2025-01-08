use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotionMark {}

impl OgfMotionMark {
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<Self> {
    todo!("Implement")
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
