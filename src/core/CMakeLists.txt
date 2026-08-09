add_library(vault_core STATIC
  crypto.cpp
  vault.cpp
)

target_include_directories(vault_core PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_include_directories(vault_core PRIVATE ${SODIUM_INCLUDE_DIRS})
target_link_libraries(vault_core PUBLIC ${SODIUM_LIBRARIES})
target_compile_options(vault_core PRIVATE ${SODIUM_CFLAGS_OTHER})
