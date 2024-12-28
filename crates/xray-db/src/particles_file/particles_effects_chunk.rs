use crate::chunk::reader::ChunkReader;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesEffectsChunk {}

impl ParticlesEffectsChunk {
  pub const CHUNK_ID: u32 = 3;

  /// Read effects chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(mut reader: ChunkReader) -> io::Result<ParticlesEffectsChunk> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);

    log::info!(
      "Parsed effects chunk, {:?} bytes, {:?} chunks",
      reader.read_bytes_len(),
      chunks.len()
    );

    // assert!(reader.is_ended(), "Expect effects chunk to be ended");

    Ok(ParticlesEffectsChunk {})
  }
}

/*
  todo:

use constant PED_CHUNK_VERSION => 1;
use constant PED_CHUNK_NAME => 2;
use constant PED_CHUNK_EFFECTDATA => 3;
use constant PED_CHUNK_ACTIONS => 4;
use constant PED_CHUNK_FLAGS => 5;
use constant PED_CHUNK_FRAME => 6;
use constant PED_CHUNK_SPRITE => 7;
use constant PED_CHUNK_TIMELIMIT => 8;
use constant PED_CHUNK_COLLISION => 33;
use constant PED_CHUNK_VEL_SCALE => 34;
use constant PED_CHUNK_DESC => 35;

use constant PED_CHUNK_UNK  => 36;

use constant PED_CHUNK_DEF_ROTATION => 37;

use constant  PAAvoidID        => 0;
use constant  PABounceID       => 1;
use constant  PACallActionListID_obsolette  => 2;
use constant  PACopyVertexBID  => 3;
use constant  PADampingID      => 4;
use constant  PAExplosionID    => 5;
use constant  PAFollowID       => 6;
use constant  PAGravitateID    => 7;
use constant  PAGravityID      => 8;
use constant  PAJetID          => 9;
use constant  PAKillOldID      => 0x0A;
use constant  PAMatchVelocityID  => 0x0B;
use constant  PAMoveID         => 0x0C;
use constant  PAOrbitLineID    => 0x0D;
use constant  PAOrbitPointID   => 0x0E;
use constant  PARandomAccelID  => 0x0F;
use constant  PARandomDisplaceID  => 0x10;
use constant  PARandomVelocityID  => 0x11;
use constant  PARestoreID      => 0x12;
use constant  PASinkID         => 0x13;
use constant  PASinkVelocityID  => 0x14;
use constant  PASourceID       => 0x15;
use constant  PASpeedLimitID   => 0x16;
use constant  PATargetColorID  => 0x17;
use constant  PATargetSizeID   => 0x18;
use constant  PATargetRotateID  => 0x19;
use constant  PATargetRotateDID  => 0x1A;
use constant  PATargetVelocityID  => 0x1B;
use constant  PATargetVelocityDID  => 0x1C;
use constant  PAVortexID       => 0x1D;
use constant  PATurbulenceID   => 0x1E;
use constant  PAScatterID      => 0x1F;
use constant  action_enum_force_dword  => 0xFFFFFFFF;

use constant FL_SOC => 0x2;

sub new {
  my $class = shift;
  my $self = {};
  $self->{service_flags} = 0;
  $self->{data} = '';
  $self->{data} = $_[0] if $#_ == 0;
  bless $self, $class;
  return $self;
}
sub read {
  my $self = shift;
  my ($mode) = @_;
  my $CDH = stkutils::chunked->new($self->{data}, 'data');
  while (1) {
    my ($index, $size) = $CDH->r_chunk_open();
    defined $index or last;
#		last if (($mode eq 'bin') && $index > PED_CHUNK_NAME);
    SWITCH: {
      $index == PED_CHUNK_VERSION && do{$self->read_version($CDH);last SWITCH;};
      $index == PED_CHUNK_NAME && do{$self->read_name($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PED_CHUNK_EFFECTDATA && do{$self->read_effectdata ($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PED_CHUNK_ACTIONS && do{$self->read_actions($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PED_CHUNK_FLAGS && do{$self->read_flags($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PED_CHUNK_FRAME && do{$self->read_frame($CDH);last SWITCH;};
      $index == PED_CHUNK_SPRITE && do{$self->read_sprite($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PED_CHUNK_TIMELIMIT && do{$self->read_timelimit($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PED_CHUNK_COLLISION && do{$self->read_collision($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PED_CHUNK_VEL_SCALE && do{$self->read_vel_scale($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PED_CHUNK_DESC && do{$self->read_description($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PED_CHUNK_DEF_ROTATION && do{$self->read_def_rotation($CDH);last SWITCH;};
      fail('unknown chunk index '.$index) if ($mode eq 'ltx');
    }
    $CDH->r_chunk_close();
  }
  $CDH->close();
}
sub read_version {
  my $self = shift;
  my ($CDH) = @_;
  $self->{version} = unpack('v', ${$CDH->r_chunk_data()});
  fail('unsupported version '.$self->{version}) unless $self->{version} == 1;
}
sub read_name {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_name}) = $packet->unpack('Z*');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_effectdata {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_MaxParticles}) = $packet->unpack('V');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_actions {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  my ($count) = $packet->unpack('V', 4);
  for (my $i = 0; $i < $count; $i++) {
    my ($type) = $packet->unpack('V', 4);
    my $action = {};
    SWITCH: {
#			$type == PAAvoidID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PABounceID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PACallActionListID_obsolette && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PACopyVertexBID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PADampingID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAExplosionID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAFollowID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAGravitateID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAGravityID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAJetID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAKillOldID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAMatchVelocityID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAMoveID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAOrbitLineID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAOrbitPointID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PARandomAccelID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PARandomDisplaceID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PARandomVelocityID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PARestoreID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PASinkID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PASinkVelocityID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
      $type == PASourceID && do {$action = pa_source->new(); $action->read($packet); last SWITCH;};
#			$type == PASpeedLimitID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PATargetColorID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PATargetSizeID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PATargetRotateID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PATargetRotateDID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PATargetVelocityID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PATargetVelocityDID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAVortexID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PATurbulenceID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
#			$type == PAScatterID && do {$action = pa_avoid->new(); $action->load($packet); last SWITCH;};
      fail('unknown type '.$type);
    }
    push @{$self->{m_Actions}}, $action;
  }
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_flags {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_Flags}) = $packet->unpack('V');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_frame {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  @{$self->{m_fTexSize}} = $packet->unpack('f2', 8);
  @{$self->{reserved}} = $packet->unpack('f2', 8);
  ($self->{m_iFrameDimX},
  $self->{m_iFrameCount},
  $self->{m_fSpeed}) = $packet->unpack('VVf', 12);
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_sprite {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_ShaderName},
  $self->{m_TextureName}) = $packet->unpack('Z*Z*');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_timelimit {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_fTimeLimit}) = $packet->unpack('f');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_collision {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_fCollideOneMinusFriction},
  $self->{m_fCollideResilience},
  $self->{m_fCollideSqrCutoff}) = $packet->unpack('fff');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_vel_scale {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  @{$self->{m_VelocityScale}} = $packet->unpack('f3');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_description {
  my $self = shift;
  my ($CDH) = @_;
  $self->{service_flags} |= FL_SOC;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_Creator},
  $self->{m_Editor},
  $self->{m_CreateTime},
  $self->{m_EditTime}) = $packet->unpack('Z*Z*VV');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_def_rotation {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  @{$self->{m_APDefaultRotation}} = $packet->unpack('f3');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub write {
  my $self = shift;
  my ($CDH, $mode, $index) = @_;

  if ($mode eq 'bin') {
    $CDH->w_chunk($index, ${$self->{data}});
  } elsif ($mode eq 'ltx') {
    $CDH->w_chunk_open($index);
    $self->write_version($CDH);
    $self->write_name($CDH);
    $self->write_effectdata($CDH);
    $self->write_actions($CDH);
    $self->write_flags($CDH);
    $self->write_sprite($CDH) if (($self->{m_Flags} & 0x1) == 0x1);
    $self->write_frame($CDH) if (($self->{m_Flags} & 0x400) == 0x400);
    $self->write_timelimit($CDH) if (($self->{m_Flags} & 0x4000) == 0x4000);
    $self->write_collision($CDH) if (($self->{m_Flags} & 0x10000) == 0x10000);
    $self->write_vel_scale($CDH) if (($self->{m_Flags} & 0x40000) == 0x40000);
    $self->write_def_rotation($CDH) if (($self->{m_Flags} & 0x8000) == 0x8000);
    $self->write_description($CDH) if (($self->{service_flags} & FL_SOC) == FL_SOC);
    $CDH->w_chunk_close();
  }
}
sub export {
  my $self = shift;
  my ($mode) = @_;

  my @path = split(/\\/, $self->{m_name});
  pop @path;
  my $path = join('\\', @path);
  File::Path::mkpath($path, 0);

  if ($mode eq 'bin') {
#		print "$self->{m_name}\n";
    my $fh = IO::File->new($self->{m_name}.'.pe', 'w');
    if (!defined $fh)
    {
      return;
    }
    binmode $fh;
    $fh->write(${$self->{data}}, length(${$self->{data}}));
    $fh->close();
  } elsif ($mode eq 'ltx') {
    my $fh = IO::File->new($self->{m_name}.'_effect.ltx', 'w');
    print $fh "[general]\n";
    $self->export_version($fh);
    $self->export_name($fh);
    $self->export_effectdata($fh);
    $self->export_actions($fh);
    $self->export_flags($fh);
    $self->export_sprite($fh) if (($self->{m_Flags} & 0x1) == 0x1);
    $self->export_frame($fh) if (($self->{m_Flags} & 0x400) == 0x400);
    $self->export_timelimit($fh) if (($self->{m_Flags} & 0x4000) == 0x4000);
    $self->export_collision($fh) if (($self->{m_Flags} & 0x10000) == 0x10000);
    $self->export_vel_scale($fh) if (($self->{m_Flags} & 0x40000) == 0x40000);
    $self->export_def_rotation($fh) if (($self->{m_Flags} & 0x8000) == 0x8000);
    $self->export_description($fh) if (($self->{service_flags} & FL_SOC) == FL_SOC);
    $fh->close();
  }
}
sub import {
  my $self = shift;
  my ($path, $mode) = @_;
  if ($mode eq 'bin') {
    $self->{m_name} = substr($path, 0, -3);
    my $fh = IO::File->new($path, 'r');
    binmode $fh;
    my $data = '';
    $fh->read($data, ($fh->stat())[7]);
    $self->{data} = \$data;
    $fh->close();
  } elsif ($mode eq 'ltx') {
    my $fh = stkutils::ini_file->new($path, 'r');
    $self->import_version($fh);
    $self->import_name($fh);
    $self->import_effectdata($fh);
    $self->import_actions($fh);
    $self->import_flags($fh);
    $self->import_sprite($fh) if (($self->{m_Flags} & 0x1) == 0x1);
    $self->import_frame($fh) if (($self->{m_Flags} & 0x400) == 0x400);
    $self->import_timelimit($fh) if (($self->{m_Flags} & 0x4000) == 0x4000);
    $self->import_collision($fh) if (($self->{m_Flags} & 0x10000) == 0x10000);
    $self->import_vel_scale($fh) if (($self->{m_Flags} & 0x40000) == 0x40000);
    $self->import_def_rotation($fh) if (($self->{m_Flags} & 0x8000) == 0x8000);
    $self->import_description($fh) if (($self->{service_flags} & FL_SOC) == FL_SOC);
    $fh->close();
  }
}
 */

/*
package pa_source;
use strict;
sub new {
  my $class = shift;
  my $self = {};
  bless $self, $class;
  return $self;
}
sub read {
  my $self = shift;
  my ($packet) = @_;
  ($self->{m_Flags},
  $self->{type}) = $packet->unpack('VV', 8);
  @{$self->{position}} = $packet->unpack('Vf16', 68);
  @{$self->{velocity}} = $packet->unpack('Vf16', 68);
  @{$self->{rot}} = $packet->unpack('Vf16', 68);
  @{$self->{size}} = $packet->unpack('Vf16', 68);
  @{$self->{color}} = $packet->unpack('Vf16', 68);
  ($self->{alpha},
  $self->{particle_rate},
  $self->{age},
  $self->{age_sigma}) = $packet->unpack('ffff', 16);
  @{$self->{parent_vel}} = $packet->unpack('f3', 12);
  ($self->{parent_motion}) = $packet->unpack('f', 4);
}
##############################
1;
 */
