use crate::chunk::reader::ChunkReader;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesGroupsChunk {}

impl ParticlesGroupsChunk {
  pub const CHUNK_ID: u32 = 4;

  /// Read effects chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(mut reader: ChunkReader) -> io::Result<ParticlesGroupsChunk> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);

    log::info!(
      "Parsed groups chunk, {:?} bytes, {:?} chunks",
      reader.read_bytes_len(),
      chunks.len()
    );

    // assert!(reader.is_ended(), "Expect groups chunk to be ended");

    Ok(ParticlesGroupsChunk {})
  }
}

/*
  todo:

use constant PGD_CHUNK_VERSION => 1;
use constant PGD_CHUNK_NAME => 2;
use constant PGD_CHUNK_FLAGS => 3;
use constant PGD_CHUNK_EFFECTS => 4;
use constant PGD_CHUNK_TIMELIMIT => 5;
use constant PGD_CHUNK_DESC => 6;
use constant PGD_CHUNK_EFFECTS2 => 7;

use constant FL_OLD => 0x1;
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
#		last if (($mode eq 'bin') && $index > PGD_CHUNK_NAME);
    SWITCH: {
      $index == PGD_CHUNK_VERSION && do{$self->read_version($CDH);last SWITCH;};
      $index == PGD_CHUNK_NAME && do{$self->read_name($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PGD_CHUNK_FLAGS && do{$self->read_flags ($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PGD_CHUNK_EFFECTS && do{$self->read_effects($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PGD_CHUNK_TIMELIMIT && do{$self->read_timelimit($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PGD_CHUNK_DESC && do{$self->read_description($CDH);last SWITCH;};
      ($mode eq 'ltx') && $index == PGD_CHUNK_EFFECTS2 && do{$self->read_effects2($CDH);last SWITCH;};
      fail('unknown chunk index '.$index) if($mode eq 'ltx');
    }
    $CDH->r_chunk_close();
  }
  $CDH->close();
}
sub read_version {
  my $self = shift;
  my ($CDH) = @_;
  $self->{version} = unpack('v', ${$CDH->r_chunk_data()});
  fail('unsupported version '.$self->{version}) unless $self->{version} == 3;
}
sub read_name {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_name}) = $packet->unpack('Z*');
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_flags {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_flags}) = $packet->unpack('V', 4);
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_effects {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  my ($count) = $packet->unpack('V', 4);
  for (my $i = 0; $i < $count; $i++) {
    my $effect = {};
    ($effect->{m_EffectName},
    $effect->{m_OnPlayChildName},
    $effect->{m_OnBirthChildName},
    $effect->{m_OnDeadChildName},
    $effect->{m_Time0},
    $effect->{m_Time1},
    $effect->{m_Flags}) = $packet->unpack('Z*Z*Z*Z*ffV');
    push @{$self->{effects}}, $effect;
  }
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_effects2 {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet($CDH->r_chunk_data());
  $self->{service_flags} |= FL_OLD;
  my ($count) = $packet->unpack('V', 4);
  for (my $i = 0; $i < $count; $i++) {
    my $effect = {};
    ($effect->{m_EffectName},
    $effect->{m_OnPlayChildName},
    $effect->{m_Time0},
    $effect->{m_Time1},
    $effect->{m_Flags}) = $packet->unpack('Z*Z*ffV');
    push @{$self->{effects}}, $effect;
  }
  fail('data left in packet: '.$packet->resid()) unless $packet->resid() == 0;
}
sub read_timelimit {
  my $self = shift;
  my ($CDH) = @_;
  my $packet = stkutils::data_packet->new($CDH->r_chunk_data());
  ($self->{m_fTimeLimit}) = $packet->unpack('f', 4);
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
sub write {
  my $self = shift;
  my ($CDH, $mode, $index) = @_;

  if ($mode eq 'bin') {
    $CDH->w_chunk($index, ${$self->{data}});
  } elsif ($mode eq 'ltx') {
    $CDH->w_chunk_open($index);
    $self->write_version($CDH);
    $self->write_name($CDH);
    $self->write_flags($CDH);
    if (($self->{service_flags} & FL_OLD) == 0) {
      $self->write_effects($CDH);
    } else {
      $self->write_effects2($CDH);
    }
    $self->write_timelimit($CDH);
    $self->write_description($CDH) if (($self->{service_flags} & FL_SOC) != 0);
    $CDH->w_chunk_close();
  }
}
sub write_version {
  my $self = shift;
  my ($CDH) = @_;
  $CDH->w_chunk(PGD_CHUNK_VERSION, pack('v', $self->{version}));
}
sub write_name {
  my $self = shift;
  my ($CDH) = @_;
  $CDH->w_chunk(PGD_CHUNK_NAME, pack('Z*', $self->{m_name}));
}
sub write_flags {
  my $self = shift;
  my ($CDH) = @_;
  $CDH->w_chunk(PGD_CHUNK_FLAGS, pack('V', $self->{m_flags}));
}
sub write_timelimit {
  my $self = shift;
  my ($CDH) = @_;
  $CDH->w_chunk(PGD_CHUNK_TIMELIMIT, pack('f', $self->{m_fTimeLimit}));
}
sub write_effects {
  my $self = shift;
  my ($CDH) = @_;
  $CDH->w_chunk_open(PGD_CHUNK_EFFECTS);
  $CDH->w_chunk_data(pack('V', $#{$self->{effects}} + 1));
  foreach my $effect (@{$self->{effects}}) {
    $CDH->w_chunk_data(pack('Z*Z*Z*Z*ffV', $effect->{m_EffectName}, $effect->{m_OnPlayChildName}, $effect->{m_OnBirthChildName}, $effect->{m_OnDeadChildName}, $effect->{m_Time0}, $effect->{m_Time1}, $effect->{m_Flags}));
  }
  $CDH->w_chunk_close();
}
sub write_effects2 {
  my $self = shift;
  my ($CDH) = @_;
  $CDH->w_chunk_open(PGD_CHUNK_EFFECTS2);
  $CDH->w_chunk_data(pack('V', $#{$self->{effects}} + 1));
  foreach my $effect (@{$self->{effects}}) {
    $CDH->w_chunk_data(pack('Z*Z*ffV', $effect->{m_EffectName}, $effect->{m_OnPlayChildName}, $effect->{m_Time0}, $effect->{m_Time1}, $effect->{m_Flags}));
  }
  $CDH->w_chunk_close();
}
sub write_description {
  my $self = shift;
  my ($CDH) = @_;
  $CDH->w_chunk(PGD_CHUNK_DESC, pack('Z*Z*VV', $self->{m_Creator}, $self->{m_Editor}, $self->{m_CreateTime}, $self->{m_EditTime}));
}
sub export {
  my $self = shift;
  my ($mode) = @_;

  my @path = split(/\\/, $self->{m_name});
  pop @path;
  my $path = join('\\', @path);
  File::Path::mkpath($path, 0);

  if ($mode eq 'bin') {
    my $fh = IO::File->new($self->{m_name}.'.pg', 'w');
    binmode $fh;
    $fh->write(${$self->{data}}, length(${$self->{data}}));
    $fh->close();
  } elsif ($mode eq 'ltx') {
    my $fh = IO::File->new($self->{m_name}.'_group.ltx', 'w');
    print $fh "[general]\n";
    $self->export_version($fh);
    print $fh "service_flags = $self->{service_flags}\n";
    $self->export_name($fh);
    $self->export_flags($fh);
    $self->export_timelimit($fh);
    $self->export_description($fh) if (($self->{service_flags} & FL_SOC) != 0);
    print $fh "\n[effects]\n";
    $self->export_effects($fh);
    $fh->close();
  }
}
sub export_version {
  my $self = shift;
  my ($ini) = @_;
  print $ini "version = $self->{version}\n";
}
sub export_name {
  my $self = shift;
  my ($ini) = @_;
  print $ini "name = $self->{m_name}\n";
}
sub export_flags {
  my $self = shift;
  my ($ini) = @_;
  print $ini "flags = $self->{m_flags}\n";
}
sub export_effects {
  my $self = shift;
  my ($ini) = @_;
  my $i = 0;
  print $ini "effects_count = ".($#{$self->{effects}} + 1)."\n";
  foreach my $effect (@{$self->{effects}}) {
    print $ini "$i:effect_name = $effect->{m_EffectName}\n";
    print $ini "$i:on_play = $effect->{m_OnPlayChildName}\n";
    print $ini "$i:on_birth = $effect->{m_OnBirthChildName}\n" if (($self->{service_flags} & FL_OLD) == 0);
    print $ini "$i:on_dead = $effect->{m_OnDeadChildName}\n" if (($self->{service_flags} & FL_OLD) == 0);
    print $ini "$i:begin_time = $effect->{m_Time0}\n";
    print $ini "$i:end_time = $effect->{m_Time1}\n";
    print $ini "$i:flags = $effect->{m_Flags}\n\n";
    $i++;
  }
}
sub export_timelimit {
  my $self = shift;
  my ($ini) = @_;
  print $ini "timelimit = $self->{m_fTimeLimit}\n";
}
sub export_description {
  my $self = shift;
  my ($ini) = @_;
  print $ini "creator = $self->{m_Creator}\n";
  print $ini "editor = $self->{m_Editor}\n";
  print $ini "create_time = $self->{m_CreateTime}\n";
  print $ini "edit_time = $self->{m_EditTime}\n";
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
    $self->import_flags($fh);
    $self->{service_flags} = $fh->value('general', 'service_flags');
    $self->import_timelimit($fh);
    $self->import_description($fh) if (($self->{service_flags} & FL_SOC) != 0);
    $self->import_effects($fh);
    $fh->close();
  }
}
sub import_version {
  my $self = shift;
  my ($ini) = @_;
  $self->{version} = $ini->value('general', 'version');
}
sub import_name {
  my $self = shift;
  my ($ini) = @_;
  $self->{m_name} = $ini->value('general', 'name');
}
sub import_flags {
  my $self = shift;
  my ($ini) = @_;
  $self->{m_flags} = $ini->value('general', 'flags');
}
sub import_timelimit {
  my $self = shift;
  my ($ini) = @_;
  $self->{m_fTimeLimit} = $ini->value('general', 'timelimit');
}
sub import_description {
  my $self = shift;
  my ($ini) = @_;
  $self->{m_Creator} = $ini->value('general', 'creator');
  $self->{m_Editor} = $ini->value('general', 'editor');
  $self->{m_CreateTime} = $ini->value('general', 'create_time');
  $self->{m_EditTime} = $ini->value('general', 'edit_time');
}
sub import_effects {
  my $self = shift;
  my ($ini) = @_;
  my $count = $ini->value('effects', 'effects_count');
  for (my $i = 0; $i < $count; $i++){
    my $effect = {};
    $effect->{m_EffectName} = $ini->value('effects', "$i:effect_name");
    $effect->{m_OnPlayChildName} = $ini->value('effects', "$i:on_play");
    $effect->{m_OnBirthChildName} = $ini->value('effects', "$i:on_birth");
    $effect->{m_OnDeadChildName} = $ini->value('effects', "$i:on_dead");
    $effect->{m_Time0} = $ini->value('effects', "$i:begin_time");
    $effect->{m_Time1} = $ini->value('effects', "$i:end_time");
    $effect->{m_Flags} = $ini->value('effects', "$i:flags");
    push @{$self->{effects}}, $effect;
  }
}
 */
