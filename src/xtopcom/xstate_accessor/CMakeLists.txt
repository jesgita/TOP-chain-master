add_compile_options(-Wpedantic)

aux_source_directory(./src src_dir)
add_library(xstate_accessor STATIC ${src_dir})

add_dependencies(xstate_accessor xdata xcommon xconfig xbasic xxbase)

target_link_libraries(xstate_accessor PRIVATE xdata xcommon xconfig xbasic xxbase pthread)
