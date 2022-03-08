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
        return f'{tool} is installed'
    else: 
        return f'{tool} is not installed. Aborting instalation'
        

if __name__ == "__main__":
    # programs that need to be installed to run this program 
    pre_req_programs = ["git", "rustup"]

    # print system information
    print_info()
    
    # check if programs are installed
    for i in pre_req_programs: 
        _bool = check_pre_req_for_install(i)
        print(_bool)

