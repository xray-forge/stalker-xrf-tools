use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotion {}

impl OgfMotion {
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<Self> {
    todo!("Implement")
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}

/*
sub read_motion {
  my $self = shift;
  my ($cf, $mode) = @_;
  my $packet = stkutils::data_packet->new($cf->r_chunk_data());
  ($self->{name}, $self->{keys_count}) = $packet->unpack('Z*V');
  print "$self->{name}, $self->{keys_count}\n";
  if ($self->{keys_count} == 0) {
    fail('there some data in packet left: '.$packet->resid()) unless $packet->resid() == 0;
    return;
  }
  if ($mode == 1) {
    for (my $i = 0; $packet->resid() > 0; $i++) {
      my @keyst;
      my $bone = {};
      ($bone->{flags}) = $packet->unpack('C', 1);
      print "$i:$bone->{flags}\n";
      fail('flags didnot match:'.$bone->{flags}) unless ($bone->{flags} & ~0x07) == 0;
      if ($bone->{flags} & KPF_R_ABSENT) {
        @{$bone->{keysr}} = $packet->unpack('s4', 4);
        print "keysr = ".join(',', @{$bone->{keysr}})."\n";
      } else {
        ($bone->{crc_keysr}, @{$bone->{keysr}}) = $packet->unpack("V(s4)$self->{keys_count}", 4+4*$self->{keys_count});
        print "keysr = $bone->{crc_keysr}, ".join(',', @{$bone->{keysr}})."\n";
      }
  #		dequantize_qr(\@{$bone->{keysr}});
      if ($bone->{flags} & KPF_T_PRESENT) {
        ($bone->{crc_keyst}) = $packet->unpack('V', 4);
        if ($bone->{flags} & KPF_T_HQ) {
          @{$bone->{keyst}} = $packet->unpack("(s3)$self->{keys_count}", 3*$self->{keys_count});
          print "keyst = ".join(',', @{$bone->{keyst}})."\n";
        } else {
          @{$bone->{keyst}} = $packet->unpack("(c3)$self->{keys_count}", 3*$self->{keys_count});
          print "keyst = ".join(',', @{$bone->{keyst}})."\n";
        }
        @{$bone->{sizet}} = $packet->unpack('f3', 12);
        print "sizet = ".join(',', @{$bone->{sizet}})."\n";
      } else {
        die unless ($bone->{flags} & KPF_T_HQ) == 0;
      }
      @{$bone->{initt}} = $packet->unpack('f3', 12);
      print "initt = ".join(',', @{$bone->{initt}})."\n";
      push @{$self->{bones}}, $bone;
    }
  } else {
    for (my $i = 0; $packet->resid() > 0; $i++) {
      my $bone = {};
      @{$bone->{keys}} = $packet->unpack("(s4f3)$self->{keys_count}", 16*$self->{keys_count});
  #		dequantize_qr(\@{$bone->{keysr}});
      push @{$self->{bones}}, $bone;
    }
  }
  fail('there some data in packet left: '.$packet->resid()) unless $packet->resid() == 0;
}
 */
