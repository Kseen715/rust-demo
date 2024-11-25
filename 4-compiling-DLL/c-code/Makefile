CC = gcc
ERRORS = -Wall
# -Wall
# -Wextra
# -Werror
# -Wpedantic
# -Wno-unused-variable
# -Wfatal-errors
CFLAGS = -c $(ERRORS) -fPIC -std=c11 -O3
# CFLAGS += -D _DEBUG
BUILD_DIR = build
EXE_NAME = demodll
DLL_NAME = demodll
SOURCES = $(wildcard *.c)
SOURCES += $(wildcard *.cpp)
HEADERS = $(wildcard *.h)
HEADERS += $(wildcard *.hpp)

TARGET = 
ifeq ($(OS),Windows_NT)
TARGET = windows
else
TARGET = linux
endif

ifeq ($(TARGET),windows)
RM = cmd /c del /Q /F
COPY = copy
MOVE = move
SYS_FLAGS = -D _WIN32
EXE_EXTENSION = .exe
DLL_EXTENSION = .dll
ARCHIVE_EXTENSION = .lib
OBJECT_EXTENSION = .obj
ECHO = @echo
SYS_MSG = "Windows_NT detected!"
C_LIBS +=
else
RM = rm -rf
COPY = cp
MOVE = mv
SYS_FLAGS = -D  
EXE_EXTENSION =
DLL_EXTENSION = .so
ARCHIVE_EXTENSION = .a
OBJECT_EXTENSION = .o
ECHO = @echo
SYS_MSG = "Linux detected!"
C_LIBS +=
endif

C_INCLUDE_PATH = -I. 
C_LINKER_PATH = -L./$(BUILD_DIR)
C_LIBS = -ldemodll

all: $(EXE_NAME)

$(EXE_NAME): build_dir platform $(SOURCES) cargo_build
# Compile the sources
	$(CC) $(CFLAGS) $(C_INCLUDE_PATH) $(SYS_FLAGS) $(SOURCES) -o $(BUILD_DIR)/$(EXE_NAME)$(OBJECT_EXTENSION)
# Link the object files
	$(CC) $(BUILD_DIR)/$(EXE_NAME)$(OBJECT_EXTENSION) -o $(BUILD_DIR)/$(EXE_NAME)$(EXE_EXTENSION) $(C_LIBS) $(C_LINKER_PATH)

run: $(EXE_NAME) 
	cd $(BUILD_DIR) && $(EXE_NAME)$(EXE_EXTENSION) && cd ../

build_dir:
	@if not exist $(BUILD_DIR) mkdir $(BUILD_DIR)

platform:
	$(ECHO) $(SYS_MSG)

cargo_build: build_dir
# Build DLL and move it to BUILD_DIR
	cd ../ && (cargo build --release)
	$(MOVE) ..\target\release\$(DLL_NAME)$(DLL_EXTENSION) $(BUILD_DIR)\$(DLL_NAME)$(DLL_EXTENSION)