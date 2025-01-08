use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfVisual {}

impl OgfVisual {
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<Self> {
    todo!("Implement")
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}

/*
my $self = shift;
  my ($cf) = @_;
  $self->read_render_visual($cf);
  if ($self->{ogf_version} == 4 && $cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_GCONTAINER'})) {
    $self->read_gcontainer($cf);
    $cf->close_found_chunk();
    if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_FASTPATH'})) {
      $self->read_fastpath($cf) ;
      $cf->close_found_chunk();
    }
    return;
  }
  if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_VCONTAINER'})) {
    $self->read_vcontainer($cf);
    $cf->close_found_chunk();
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_VERTICES'})) {
    $self->read_vertices($cf);
    $cf->close_found_chunk();
  }
  if ($self->{ogf_version} != 2 && $cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_ICONTAINER'})) {
    $self->read_icontainer($cf);
    $cf->close_found_chunk();
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_INDICES'})) {
    $self->read_indices($cf);
    $cf->close_found_chunk();
  }
 */
