cmake_minimum_required(VERSION 3.8)

add_subdirectory(xgenerate_account)
# add_subdirectory(xtopcl_cli)
add_subdirectory(xdb_export)
add_subdirectory(xaction_param_parse)

if (STATIC_CONSENSUS)
    add_subdirectory(xstatic_consensus_config_check)
endif()

if (DB_TOOL)
    add_subdirectory(xdb_tool)
endif()
