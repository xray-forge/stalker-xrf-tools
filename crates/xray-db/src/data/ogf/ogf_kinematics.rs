use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfKinematics {}

impl OgfKinematics {
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<Self> {
    todo!("Implement")
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}

/*
sub read_kinematics {
  my $self = shift;
  my ($cf) = @_;
  $self->read_hierrarhy_visual($cf);
  if ($self->{ogf_version} == 4) {
    my $size = $cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_LODS'});
    if ($size) {
      if ($size < 0x100) {
        $self->read_s_lods_csky($cf);
      } else {
        $self->read_s_lods($cf);
      }
      $cf->close_found_chunk();
    }
  }
  if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_USERDATA'})) {
    $self->read_s_userdata($cf);
    $cf->close_found_chunk();
  }
  if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_BONE_NAMES'})) {
    $self->read_s_bone_names($cf);
    $cf->close_found_chunk();
  } else {
    fail('cannot find OGF_S_BONE_NAMES');
  }
  if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_IKDATA_2'})) {
    $self->read_s_ikdata($cf, 2);
    $cf->close_found_chunk();
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_IKDATA_1'})) {
    $self->read_s_ikdata($cf, 1);
    $cf->close_found_chunk();
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_IKDATA_0'})) {
    self->read_s_ikdata($cf, 0);
    $cf->close_found_chunk();
  }
}
 */
