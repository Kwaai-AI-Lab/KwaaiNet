# MLX refuses a target below macOS 14; the rest of the binary targets 11.0.
if(CMAKE_SOURCE_DIR MATCHES "/mlx-sys-[^/]+/src/mlx-c$")
  set(CMAKE_OSX_DEPLOYMENT_TARGET "14.0" CACHE STRING "" FORCE)
endif()
