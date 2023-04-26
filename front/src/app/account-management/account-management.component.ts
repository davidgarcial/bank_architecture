import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { CreateAccountComponent } from '../create-account/create-account.component';
import { HistoryComponent } from '../history/history.component';

@Component({
  selector: 'app-account-managment',
  templateUrl: './account-management.component.html',
  styleUrls: ['./account-management.component.scss'],
})
export class AccountManagmentComponent implements OnInit {
  selectedAccount: any;

  constructor(public dialog: MatDialog) {}

  ngOnInit(): void {}

  openCreateAccountDialog(): void {
    const dialogRef = this.dialog.open(CreateAccountComponent, {
      width: '450px',
    });

    dialogRef.afterClosed().subscribe((result) => {
      console.log('The create account dialog was closed');
      // Refresh account list if needed
    });
  }

  openTransactionHistoryDialog(account: any): void {
    const dialogRef = this.dialog.open(HistoryComponent, {
      width: '600px',
      data: { account: account },
    });

    dialogRef.afterClosed().subscribe((result) => {
      console.log('The transaction history dialog was closed');
      // Refresh account list if needed
    });
  }


  onAccountSelected(account: any) {
    this.selectedAccount = account;
  }
}
