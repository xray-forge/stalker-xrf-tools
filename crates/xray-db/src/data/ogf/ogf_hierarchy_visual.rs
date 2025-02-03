use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfHierarchyVisual {}

impl ChunkReadWrite for OgfHierarchyVisual {
  fn read<T: ByteOrder>(_: &mut ChunkReader) -> XRayResult<Self> {
    todo!("Implement")
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement")
  }
}

/*

sub read_hierrarhy_visual {
  my $self = shift;
  my ($cf) = @_;
  $self->read_render_visual($cf);
  if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_CHILDREN_L'})) {
    $self->read_children_l($cf);
  } elsif ($self->{ogf_version} != 2 && $cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_CHILDREN'})) {
    $self->read_children($cf);
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_CHILD_REFS'})) {
    $self->read_child_refs($cf);
  } else {
    fail('Invalid visual, no children');
  }
  $cf->close_found_chunk();
};
 */
