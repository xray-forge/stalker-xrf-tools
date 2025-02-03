use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfRenderVisual {}

impl ChunkReadWrite for OgfRenderVisual {
  fn read<T: ByteOrder>(_: &mut ChunkReader) -> XRayResult<Self> {
    todo!("Implement")
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement")
  }
}

/*
my $self = shift;
  my ($cf) = @_;
  if ($self->{ogf_version} == 3) {
    if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_BBOX'})) {
      my $packet = stkutils::data_packet->new($cf->r_chunk_data());
      $self->read_bbox($packet);
      $self->set_loaded('OGF_BBOX');
      $cf->close_found_chunk();
    } else {
      fail('cannot find OGF_BBOX chunk');
    }
  }
  if ($self->{ogf_version} != 4 && $cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_BSPHERE'})) {
    my $packet = stkutils::data_packet->new($cf->r_chunk_data());
    $self->read_bsphere($packet);
    $self->set_loaded('OGF_BSPHERE');
    $cf->close_found_chunk();
  }
  if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_DESC'})) {
    $self->read_s_desc($cf);
    $cf->close_found_chunk();
  }
  if ($self->{ogf_version} != 4 && $cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_TEXTURE_L'})) {
    $self->read_texture_l($cf);
    $cf->close_found_chunk();
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_TEXTURE'})) {
    $self->read_texture($cf);
    $cf->close_found_chunk();
  }
 */
