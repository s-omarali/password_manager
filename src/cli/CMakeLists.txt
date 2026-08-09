add_executable(pm_cli
  main.cpp
  commands.cpp
)

target_link_libraries(pm_cli PRIVATE vault_core)
