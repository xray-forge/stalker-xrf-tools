use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfHeader {}

impl OgfHeader {
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> XRayResult<Self> {
    todo!("Implement")
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement")
  }
}

/*
my $self = shift;
  my ($cf) = @_;
  my $packet = stkutils::data_packet->new($cf->r_chunk_data());
  ($self->{ogf_version}, $self->{model_type}, $self->{shader_id}) = $packet->unpack('CCv', 4);
  fail('unexpected ogf_file version '.$self->{ogf_version}) unless $self->{ogf_version} >= 2 && $self->{ogf_version} <= 4;
  if ($self->{ogf_version} == 4) {
    $self->read_bbox($packet);
    $self->read_bsphere($packet);
  }
  $self->set_loaded('OGF_HEADER');
 */
