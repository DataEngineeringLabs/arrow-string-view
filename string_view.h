#include "stdint.h"

typedef struct StringView {
  /// Length of the string
  uint32_t size_;

  /// The first 4 bytes of the string, whether inline or heap
  char prefix_[4];

  union {
    /// For string 12 bytes or less, the remaining bytes beyond the first 4
    char remainder_inlined[8];
    
    /// For strings over 12 bytes in length (points to first byte of string)
    const char* complete_string_data;
  } value_;
};
