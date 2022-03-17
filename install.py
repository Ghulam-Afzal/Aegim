from shutil import which
import platform
import sys 

# check what linux distro it is if applicable
def linux_distribution():
  try:
    return platform.linux_distribution()
  except:
    return "N/A"


# check and pring system infor
def print_info(): 
    print("""Python version: %s
    linux_distribution: %s
    system: %s
    machine: %s
    platform: %s
    uname: %s
    version: %s
    mac_ver: %s
    """ % (
        sys.version.split('\n'),
        linux_distribution(),
        platform.system(),
        platform.machine(),
        platform.platform(),
        platform.uname(),
        platform.version(),
        platform.mac_ver(),
    ))


# check whether required programs are installed or not
def check_pre_req_for_install(tool):
    _bool = which(tool) is not None

    if _bool is True: 
        return 0
    else: 
        return 1 
        

if __name__ == "__main__":

    # programs that need to be installed to run this program 
    pre_req_programs = ["git", "rustup"]

    # print system information
    print_info()
    
    # check if programs are installed
    for i in pre_req_programs: 
        _bool = check_pre_req_for_install(i)

        # exit the program if not otherwise check the next program
        if _bool != 1: 
            print(f"{i} is installed")

        else: 
            print(f"{i} is not installed and program will abort the installation")
            exit()

