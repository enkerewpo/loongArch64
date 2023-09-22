use core::fmt;
use bit_field::BitField;
use crate::PALEN;
use crate::register::MemoryAccessType;

impl_read_csr!(0x12,TlbElo0);
impl_define_csr!(TlbElo0, "TLB Entry Low-order Bits
n
TLBELO0 and TLBELO1 registers contain the information related to the physical page number of the low-order bits of the TLB table entry during executing TLB-related instructions.
Since TLB adopts a dual-page structure,
the low-order bits of TLB table entry corresponds to the odd and even physical page table entries,
where the even page information is in TLBELO0 and the odd page information is in TLBELO1.
TLBELO0 and TLBELO1 registers have exactly the same format definition

When CSR.TLBRERA.IsTLBR=0, and when executing the TLBWR and TLBFILL instructions,
and the written values of the G, PFN, V, PLV, MAT, D, NR, NX, RPLV fields of the TLB table entry come from TLBELOO and TLBELO1 fields, respectively.

When executing the TLBRD instruction,
the above information read from the TLB table entry is written to the corresponding fields in the TLBELO0 and TLBELO1 registers one by one.
");

impl_tlbelo!(TlbElo0, 0x12);

impl fmt::Display for TlbElo0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TlbElo0: RPLV:{},NX:{},NR:{},PPN:{:#x},G:{},MAT:{},PLV:{},D:{},V:{}",
            self.rplv(),
            self.not_executable(),
            self.not_readable(),
            self.ppn(),
            self.global(),
            self.mat(),
            self.plv(),
            self.dirty(),
            self.valid()
        )
    }
}
