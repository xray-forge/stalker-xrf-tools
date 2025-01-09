use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfProgressive {}

impl OgfProgressive {
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<Self> {
    todo!("Implement")
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}

/*
sub read_progressive {
  my $self = shift;
  my ($cf) = @_;
  $self->read_visual($cf);
  if ($self->{ogf_version} == 4 && $cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_SWIDATA'})) {
    $self->read_swidata($cf);
    $cf->close_found_chunk();
  } else {
    if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_LODDATA'})) {
      $self->read_loddata($cf);
      $cf->close_found_chunk();
    } else {
      fail('Invalid visual, no loddata');
    }
  }
}
 */
