cmake_minimum_required(VERSION 3.8)

aux_source_directory(./src xapplication_src)
add_library(xapplication STATIC ${xapplication_src})


add_definitions(
    -DUSE_STANDALONE_ASIO
    -DASIO_STANDALONE
    # -D__STRICT_ANSI__
)

add_dependencies(xapplication xvnode xloader xconfig xelect xelect_net xchain_timer xvnetwork xelection xblockmaker xtxexecutor xunit_service xtxpoolsvr_v2 xtxstore xBFT xsync xtopcl xmbus xbasic xcertauth xsystem_contract_runtime)
IF (CMAKE_SYSTEM_NAME MATCHES "Darwin")
    target_link_libraries(xapplication PRIVATE xvnode xloader xconfig xelect xelect_net xchain_timer xvnetwork xgenesis xblockmaker xtxexecutor xunit_service xtxpoolsvr_v2 xtxstore xBFT xelection xrpc xsync xtopcl xmbus xbasic xcertauth xsystem_contract_runtime)
ELSE()
    target_link_libraries(xapplication PRIVATE xvnode xloader xconfig xelect xelect_net xchain_timer xvnetwork xgenesis xblockmaker xtxexecutor xunit_service xtxpoolsvr_v2 xtxstore xBFT xelection xrpc xsync xtopcl xmbus xbasic xcertauth xsystem_contract_runtime intel_aes64)
ENDIF()
