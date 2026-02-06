module matrix
    use, intrinsic :: iso_fortran_env, only: real32, real64
    implicit none

    contains

    ! print the elements of a matrix to stdout
    subroutine print_matrix(matrix)
        implicit none
        ! the dimensions of the matrix are known at call-time
        real(real64), intent(in) :: matrix(:,:)
        integer(real64) :: i, j

        print ("Printing", I1, "rows and", I1, " cols"), size(matrix, 1), size(matrix, 2)

        rows: do i = 1, size(matrix, 1)
            cols: do j = 1, size(matrix, 2)
                print *, matrix(i, j)
            end do cols
        end do rows
    end subroutine print_matrix

end module matrix
