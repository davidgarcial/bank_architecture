import { AccountService } from  '../_services/account.service';
import { Component, OnInit, Output, EventEmitter } from '@angular/core';

@Component({
  selector: 'app-account-list',
  templateUrl: './account-list.component.html',
  styleUrls: ['./account-list.component.scss'],
})
export class AccountListComponent implements OnInit {
  @Output() accountSelected = new EventEmitter<any>();
  accounts: any[] = [];

  constructor(private accountService: AccountService) {}

  ngOnInit(): void {
    this.accountService.getAccounts().subscribe(
      (data) => {
        this.accounts = data.accounts;
        console.log(this.accounts);
      },
      (error) => {
        console.log(error);
      }
    );
  }

  selectAccount(account: any): void {
    this.accountSelected.emit(account);
  }
}
