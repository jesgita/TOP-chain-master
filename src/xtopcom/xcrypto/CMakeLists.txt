add_compile_options(-static)

aux_source_directory(./src src_dir)
add_library(xcrypto STATIC ${src_dir})

#add_dependencies(xcrypto xxbase)

target_link_libraries(xcrypto PRIVATE trezor-crypto xutility xxbase secp256k1 pthread xpbase)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    if (XENABLE_CODE_COVERAGE)
        target_link_libraries(xcrypto PRIVATE gcov)
    endif()
endif()

if (BUILD_METRICS)
    target_link_libraries(xcrypto PRIVATE xmetrics)
endif()

